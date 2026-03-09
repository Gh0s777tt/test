//! # Spinlock Module
//! 
//! Implementuje spinlocki i read-write locki dla synchronizacji w środowisku SMP.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::ops::{Deref, DerefMut};

/// Spinlock - prosty mutex oparty na aktywnym czekaniu
pub struct SpinLock<T> {
    /// Flaga zajętości
    locked: AtomicBool,
    /// Chronione dane
    data: core::cell::UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// Tworzy nowy spinlock
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
        }
    }
    
    /// Pobiera blokadę
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Aktywne czekanie
            core::hint::spin_loop();
        }
        
        SpinLockGuard {
            lock: self,
        }
    }
    
    /// Próbuje pobrać blokadę bez czekania
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(SpinLockGuard {
                lock: self,
            })
        } else {
            None
        }
    }
    
    /// Sprawdza czy blokada jest zajęta
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Acquire)
    }
}

/// Guard dla SpinLock
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

/// Read-Write Lock - pozwala na wielu czytelników lub jednego pisarza
pub struct RwLock<T> {
    /// Liczba czytelników
    readers: AtomicU32,
    /// Flaga pisarza
    writer: AtomicBool,
    /// Chronione dane
    data: core::cell::UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    /// Tworzy nowy RwLock
    pub const fn new(data: T) -> Self {
        Self {
            readers: AtomicU32::new(0),
            writer: AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
        }
    }
    
    /// Pobiera blokadę do odczytu
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            // Sprawdź czy nie ma pisarza
            if !self.writer.load(Ordering::Acquire) {
                // Zwiększ licznik czytelników
                let old_readers = self.readers.fetch_add(1, Ordering::AcqRel);
                
                // Sprawdź ponownie czy nie ma pisarza (race condition)
                if !self.writer.load(Ordering::Acquire) {
                    return RwLockReadGuard {
                        lock: self,
                    };
                } else {
                    // Pisarz wszedł - cofnij licznik
                    self.readers.fetch_sub(1, Ordering::AcqRel);
                }
            }
            
            core::hint::spin_loop();
        }
    }
    
    /// Próbuje pobrać blokadę do odczytu bez czekania
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        if !self.writer.load(Ordering::Acquire) {
            let old_readers = self.readers.fetch_add(1, Ordering::AcqRel);
            
            if !self.writer.load(Ordering::Acquire) {
                return Some(RwLockReadGuard {
                    lock: self,
                });
            } else {
                self.readers.fetch_sub(1, Ordering::AcqRel);
            }
        }
        
        None
    }
    
    /// Pobiera blokadę do zapisu
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        // Ustaw flagę pisarza
        while self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        
        // Czekaj aż wszyscy czytelnicy skończą
        while self.readers.load(Ordering::Acquire) > 0 {
            core::hint::spin_loop();
        }
        
        RwLockWriteGuard {
            lock: self,
        }
    }
    
    /// Próbuje pobrać blokadę do zapisu bez czekania
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        if self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            if self.readers.load(Ordering::Acquire) == 0 {
                return Some(RwLockWriteGuard {
                    lock: self,
                });
            } else {
                self.writer.store(false, Ordering::Release);
            }
        }
        
        None
    }
}

/// Guard dla RwLock (odczyt)
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> Drop for RwLockReadGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.readers.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<'a, T> Deref for RwLockReadGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

/// Guard dla RwLock (zapis)
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.writer.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for RwLockWriteGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spinlock_basic() {
        let lock = SpinLock::new(42);
        {
            let guard = lock.lock();
            assert_eq!(*guard, 42);
        }
        assert!(!lock.is_locked());
    }
    
    #[test]
    fn test_spinlock_try_lock() {
        let lock = SpinLock::new(42);
        let guard = lock.try_lock().unwrap();
        assert_eq!(*guard, 42);
        assert!(lock.try_lock().is_none());
        drop(guard);
        assert!(lock.try_lock().is_some());
    }
    
    #[test]
    fn test_rwlock_read() {
        let lock = RwLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
    
    #[test]
    fn test_rwlock_write() {
        let lock = RwLock::new(42);
        {
            let mut guard = lock.write();
            *guard = 100;
        }
        let guard = lock.read();
        assert_eq!(*guard, 100);
    }
}