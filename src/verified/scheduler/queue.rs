//! # Task Queue Module
//! 
//! Implementuje kolejki zadań dla planisty.

use core::sync::atomic::{AtomicU32, Ordering};

/// Typ kolejki zadań
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskQueueType {
    /// Kolejka czasu rzeczywistego
    RealTime,
    /// Kolejka normalna
    Normal,
    /// Kolejka tła
    Batch,
    /// Kolejka idle
    Idle,
}

/// Zadanie w kolejce
#[derive(Debug, Clone)]
pub struct Task {
    /// ID zadania
    pub id: u64,
    /// Priorytet
    pub priority: super::priority::Priority,
    /// Czas wykonania (ns)
    pub exec_time: u64,
    /// Czas wirtualny (vruntime)
    pub vruntime: u64,
    /// ID CPU
    pub cpu_id: u32,
    /// Stan zadania
    pub state: TaskState,
}

/// Stan zadania
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Gotowe do wykonania
    Ready,
    /// Wykonywane
    Running,
    /// Zablokowane
    Blocked,
    /// Zakończone
    Finished,
}

/// Kolejka zadań
pub struct TaskQueue {
    /// Typ kolejki
    pub queue_type: TaskQueueType,
    /// Lista zadań
    tasks: Vec<Task>,
    /// Maksymalna liczba zadań
    max_tasks: usize,
}

impl TaskQueue {
    /// Tworzy nową kolejkę zadań
    pub fn new(queue_type: TaskQueueType, max_tasks: usize) -> Self {
        Self {
            queue_type,
            tasks: Vec::new(),
            max_tasks,
        }
    }
    
    /// Dodaje zadanie do kolejki
    pub fn enqueue(&mut self, task: Task) -> Result<(), super::SchedulerError> {
        if self.tasks.len() >= self.max_tasks {
            return Err(super::SchedulerError::QueueFull);
        }
        
        self.tasks.push(task);
        Ok(())
    }
    
    /// Pobiera zadanie z kolejki
    pub fn dequeue(&mut self) -> Result<Task, super::SchedulerError> {
        if self.tasks.is_empty() {
            return Err(super::SchedulerError::QueueEmpty);
        }
        
        // Dla CFS wybierz zadanie z najmniejszym vruntime
        if self.queue_type == TaskQueueType::Normal {
            self.tasks.sort_by(|a, b| a.vruntime.cmp(&b.vruntime));
        }
        
        Ok(self.tasks.remove(0))
    }
    
    /// Pobiera zadanie bez usuwania
    pub fn peek(&self) -> Option<&Task> {
        self.tasks.first()
    }
    
    /// Zwraca liczbę zadań w kolejce
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    
    /// Sprawdza czy kolejka jest pusta
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    
    /// Usuwa zadanie z kolejki
    pub fn remove(&mut self, task_id: u64) -> Result<Task, super::SchedulerError> {
        let pos = self.tasks.iter().position(|t| t.id == task_id)
            .ok_or(super::SchedulerError::QueueEmpty)?;
        
        Ok(self.tasks.remove(pos))
    }
    
    /// Aktualizuje zadanie w kolejce
    pub fn update(&mut self, task: Task) -> Result<(), super::SchedulerError> {
        let pos = self.tasks.iter().position(|t| t.id == task.id)
            .ok_or(super::SchedulerError::QueueEmpty)?;
        
        self.tasks[pos] = task;
        Ok(())
    }
}

/// Menedżer kolejek zadań
pub struct TaskQueueManager {
    /// Kolejka czasu rzeczywistego
    realtime_queue: TaskQueue,
    /// Kolejka normalna
    normal_queue: TaskQueue,
    /// Kolejka tła
    batch_queue: TaskQueue,
    /// Kolejka idle
    idle_queue: TaskQueue,
}

impl TaskQueueManager {
    /// Tworzy nowy menedżer kolejek
    pub fn new(max_tasks: usize) -> Self {
        Self {
            realtime_queue: TaskQueue::new(TaskQueueType::RealTime, max_tasks),
            normal_queue: TaskQueue::new(TaskQueueType::Normal, max_tasks),
            batch_queue: TaskQueue::new(TaskQueueType::Batch, max_tasks),
            idle_queue: TaskQueue::new(TaskQueueType::Idle, max_tasks),
        }
    }
    
    /// Dodaje zadanie do odpowiedniej kolejki
    pub fn enqueue(&mut self, task: Task) -> Result<(), super::SchedulerError> {
        let queue = self.get_queue_mut(task.priority.level)?;
        queue.enqueue(task)
    }
    
    /// Pobiera zadanie z kolejki
    pub fn dequeue(&mut self, priority_level: super::priority::PriorityLevel) -> Result<Task, super::SchedulerError> {
        let queue = self.get_queue_mut(priority_level)?;
        queue.dequeue()
    }
    
    /// Pobiera kolejne zadanie do wykonania
    pub fn next_task(&mut self) -> Result<Task, super::SchedulerError> {
        // Najpierw sprawdź kolejkę czasu rzeczywistego
        if !self.realtime_queue.is_empty() {
            return self.realtime_queue.dequeue();
        }
        
        // Potem kolejkę normalną
        if !self.normal_queue.is_empty() {
            return self.normal_queue.dequeue();
        }
        
        // Potem kolejkę tła
        if !self.batch_queue.is_empty() {
            return self.batch_queue.dequeue();
        }
        
        // Na końcu kolejkę idle
        if !self.idle_queue.is_empty() {
            return self.idle_queue.dequeue();
        }
        
        Err(super::SchedulerError::QueueEmpty)
    }
    
    /// Zwraca kolejkę dla danego poziomu priorytetu
    fn get_queue(&self, level: super::priority::PriorityLevel) -> Result<&TaskQueue, super::SchedulerError> {
        match level {
            super::priority::PriorityLevel::RealTime => Ok(&self.realtime_queue),
            super::priority::PriorityLevel::High => Ok(&self.normal_queue),
            super::priority::PriorityLevel::Normal => Ok(&self.normal_queue),
            super::priority::PriorityLevel::Low => Ok(&self.batch_queue),
            super::priority::PriorityLevel::Idle => Ok(&self.idle_queue),
        }
    }
    
    /// Zwraca mutowalną kolejkę dla danego poziomu priorytetu
    fn get_queue_mut(&mut self, level: super::priority::PriorityLevel) -> Result<&mut TaskQueue, super::SchedulerError> {
        match level {
            super::priority::PriorityLevel::RealTime => Ok(&mut self.realtime_queue),
            super::priority::PriorityLevel::High => Ok(&mut self.normal_queue),
            super::priority::PriorityLevel::Normal => Ok(&mut self.normal_queue),
            super::priority::PriorityLevel::Low => Ok(&mut self.batch_queue),
            super::priority::PriorityLevel::Idle => Ok(&mut self.idle_queue),
        }
    }
    
    /// Zwraca statystyki kolejek
    pub fn stats(&self) -> TaskQueueStats {
        TaskQueueStats {
            realtime_count: self.realtime_queue.len(),
            normal_count: self.normal_queue.len(),
            batch_count: self.batch_queue.len(),
            idle_count: self.idle_queue.len(),
            total_count: self.realtime_queue.len() + self.normal_queue.len() +
                        self.batch_queue.len() + self.idle_queue.len(),
        }
    }
}

/// Statystyki kolejek zadań
#[derive(Debug, Clone)]
pub struct TaskQueueStats {
    /// Liczba zadań w kolejce czasu rzeczywistego
    pub realtime_count: usize,
    /// Liczba zadań w kolejce normalnej
    pub normal_count: usize,
    /// Liczba zadań w kolejce tła
    pub batch_count: usize,
    /// Liczba zadań w kolejce idle
    pub idle_count: usize,
    /// Całkowita liczba zadań
    pub total_count: usize,
}

/// Globalny menedżer kolejek zadań
static QUEUE_MANAGER: std::sync::OnceLock<std::sync::RwLock<TaskQueueManager>> = std::sync::OnceLock::new();

fn get_queue_manager() -> &'static std::sync::RwLock<TaskQueueManager> {
    QUEUE_MANAGER.get_or_init(|| std::sync::RwLock::new(TaskQueueManager::new(1000)))
}

/// Inicjalizuje kolejki zadań
pub fn init() -> Result<(), super::SchedulerError> {
    // Placeholder - inicjalizacja kolejek
    Ok(())
}

/// Dodaje zadanie do kolejki
pub fn enqueue_task(task: Task) -> Result<(), super::SchedulerError> {
    let mut manager = get_queue_manager().write()
        .map_err(|_| super::SchedulerError::QueueFull)?;
    manager.enqueue(task)
}

/// Pobiera zadanie z kolejki
pub fn dequeue_task(priority_level: super::priority::PriorityLevel) -> Result<Task, super::SchedulerError> {
    let mut manager = get_queue_manager().write()
        .map_err(|_| super::SchedulerError::QueueEmpty)?;
    manager.dequeue(priority_level)
}

/// Pobiera kolejne zadanie do wykonania
pub fn next_task() -> Result<Task, super::SchedulerError> {
    let mut manager = get_queue_manager().write()
        .map_err(|_| super::SchedulerError::QueueEmpty)?;
    manager.next_task()
}

/// Zwraca statystyki kolejek
pub fn queue_stats() -> TaskQueueStats {
    let manager = get_queue_manager().read().unwrap();
    manager.stats()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_queue_creation() {
        let queue = TaskQueue::new(TaskQueueType::Normal, 100);
        assert_eq!(queue.queue_type, TaskQueueType::Normal);
        assert_eq!(queue.max_tasks, 100);
        assert!(queue.is_empty());
    }
    
    #[test]
    fn test_task_queue_enqueue_dequeue() {
        let mut queue = TaskQueue::new(TaskQueueType::Normal, 100);
        
        let task = Task {
            id: 1,
            priority: super::priority::Priority::normal(),
            exec_time: 1000,
            vruntime: 0,
            cpu_id: 0,
            state: TaskState::Ready,
        };
        
        queue.enqueue(task.clone()).unwrap();
        assert_eq!(queue.len(), 1);
        
        let dequeued = queue.dequeue().unwrap();
        assert_eq!(dequeued.id, 1);
        assert!(queue.is_empty());
    }
}