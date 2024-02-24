"""
Simplified script to check for duplicate translation keys in specified or all files within the `lang` and `original` directories.
"""

import argparse
import os
from collections import defaultdict
from typing import List


def check_duplicate_key(list_file: List[str]):
    duplicate_keys = defaultdict(list)
    for file_path in list_file:
        with open(file_path, 'r', encoding='utf-8') as file:
            for line in file:
                key = line.split('=')[0].strip()
                duplicate_keys[key].append(file_path)
    
    for key, files in duplicate_keys.items():
        if len(files) > 1:
            if key in ['', 'Version 2 30']:
                continue
            print(f"Duplicate key '{key}' found in files: {', '.join(files)}")

def main():
    """
    Main function to parse arguments and check for duplicate keys.
    """
    parser = argparse.ArgumentParser(description='Check for duplicate translation keys in specified or all files within the `lang` and `original` directories.')
    parser.add_argument('--file', dest='file_path', help='The file path to check for duplicates', required=False)
    args = parser.parse_args()
    
    if args.file_path:
        if not os.path.exists(args.file_path):
            print(f'File {args.file_path} does not exist')
            return
        if os.path.isdir(args.file_path):
            print(f'File {args.file_path} is a directory')
            return
        list_file = [args.file_path]
    else:
        if not os.path.exists('lang'):
            print('Directory `lang` does not exist')
            return
        list_file = [os.path.join('lang', file) for file in os.listdir('lang') if file.endswith('.oxt')]
    
    check_duplicate_key(list_file)

if __name__ == "__main__":
    main()