#!/usr/bin/env python3
"""
Skrypt do automatycznej migracji składni Verus
z starej wersji (builtin::*) do nowej (verus! macro)
"""

import re
import sys
from pathlib import Path

def migrate_file(file_path):
    """Migruje pojedynczy plik do nowej składni Verus"""
    
    print(f"\n🔄 Migracja: {file_path}")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Backup oryginalnego pliku
    backup_path = file_path.with_suffix('.rs.backup')
    with open(backup_path, 'w') as f:
        f.write(content)
    print(f"   📦 Backup: {backup_path}")
    
    # 1. Usuń stare importy Verus (z #[cfg(feature = "verus")])
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\nuse builtin::\*;\s*\n',
        '',
        content
    )
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\nuse builtin_macros::\*;\s*\n',
        '',
        content
    )
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\nuse vstd::prelude::\*;\s*\n',
        '',
        content
    )
    
    # 2. Dodaj nowy import na początku (po komentarzach dokumentacyjnych)
    lines = content.split('\n')
    insert_index = 0
    
    # Znajdź koniec komentarzy dokumentacyjnych
    in_doc_comment = False
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('//!'):
            in_doc_comment = True
            continue
        elif in_doc_comment and not stripped.startswith('//'):
            insert_index = i
            break
        elif not stripped.startswith('//') and stripped:
            insert_index = i
            break
    
    # Dodaj import jeśli nie istnieje
    if 'use vstd::prelude::*;' not in content:
        lines.insert(insert_index, 'use vstd::prelude::*;\n')
        print(f"   ✅ Dodano: use vstd::prelude::*;")
    
    content = '\n'.join(lines)
    
    # 3. Usuń #[cfg(feature = "verus")] przed funkcjami spec/proof (pub i bez pub)
    removed_cfg = 0
    # Usuń przed pub spec/proof fn
    content_new = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\n(?=pub (spec|proof) fn)',
        '',
        content
    )
    # Usuń przed spec/proof fn (bez pub)
    content_new = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\n(?=(spec|proof) fn)',
        '',
        content_new
    )
    removed_cfg = content.count('#[cfg(feature = "verus")]') - content_new.count('#[cfg(feature = "verus")]')
    content = content_new
    if removed_cfg > 0:
        print(f"   ✅ Usunięto {removed_cfg} x #[cfg(feature = &quot;verus&quot;)]")
    
    # 4. Znajdź pierwszą funkcję spec/proof i dodaj verus! {
    # Szukaj zarówno pub spec/proof jak i spec/proof bez pub
    verus_start = re.search(r'^((pub )?(spec|proof) fn)', content, re.MULTILINE)
    if verus_start:
        insert_pos = verus_start.start()
        # Sprawdź czy już nie ma verus! {
        if 'verus!' not in content[:insert_pos]:
            content = content[:insert_pos] + 'verus! {\n\n' + content[insert_pos:]
            print(f"   ✅ Dodano: verus! {{")
    
    # 5. Dodaj } // verus! na końcu pliku (przed ostatnimi pustymi liniami)
    if 'verus!' in content and not content.rstrip().endswith('} // verus!'):
        content = content.rstrip() + '\n\n} // verus!\n'
        print(f"   ✅ Dodano: }} // verus!")
    
    # 6. Zapisz zmigrowany plik
    with open(file_path, 'w') as f:
        f.write(content)
    
    # Statystyki
    changes = len(original_content) != len(content)
    if changes:
        print(f"   ✅ Zmigrowano: {file_path}")
        print(f"   📊 Rozmiar: {len(original_content)} → {len(content)} bajtów")
    else:
        print(f"   ℹ️  Brak zmian w pliku")
    
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 migrate_verus_syntax.py <file1.rs> [file2.rs ...]")
        print("\nExample:")
        print("  python3 migrate_verus_syntax.py ipc_message_integrity.rs")
        print("  python3 migrate_verus_syntax.py ipc*.rs")
        sys.exit(1)
    
    print("🚀 Migracja składni Verus")
    print("=" * 60)
    
    success_count = 0
    error_count = 0
    
    for file_path in sys.argv[1:]:
        path = Path(file_path)
        if not path.exists():
            print(f"❌ Plik nie istnieje: {file_path}")
            error_count += 1
            continue
        
        try:
            if migrate_file(path):
                success_count += 1
        except Exception as e:
            print(f"❌ Błąd podczas migracji {file_path}: {e}")
            error_count += 1
    
    print("\n" + "=" * 60)
    print(f"📊 Podsumowanie:")
    print(f"   ✅ Sukces: {success_count}")
    print(f"   ❌ Błędy: {error_count}")
    print(f"   📦 Backupy: *.rs.backup")
    
    if error_count == 0:
        print("\n🎉 Migracja zakończona sukcesem!")
    else:
        print(f"\n⚠️  Migracja zakończona z {error_count} błędami")

if __name__ == '__main__':
    main()