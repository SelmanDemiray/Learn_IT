import React, { useState, useEffect, useRef } from 'react';
import dictionary from '../data/dictionary';

const Dictionary = ({ open, onClose }: { open: boolean, onClose: () => void }) => {
  const [search, setSearch] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handler = () => onClose ? onClose() : undefined;
    window.addEventListener('closeDictionary', handler);
    return () => window.removeEventListener('closeDictionary', handler);
  }, [onClose]);
  useEffect(() => {
    const handler = () => setSearch('');
    window.addEventListener('openDictionary', handler);
    return () => window.removeEventListener('openDictionary', handler);
  }, []);
  useEffect(() => {
    if (open && inputRef.current) inputRef.current.focus();
  }, [open]);

  useEffect(() => {
    const escHandler = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && open) onClose();
    };
    window.addEventListener('keydown', escHandler);
    return () => window.removeEventListener('keydown', escHandler);
  }, [open, onClose]);

  if (!open) return null;

  const filtered = Object.entries(dictionary)
    .filter(([term, def]) =>
      term.toLowerCase().includes(search.toLowerCase()) ||
      def.toLowerCase().includes(search.toLowerCase())
    );

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl w-full max-w-xl p-8 relative border-2 border-blue-200 dark:border-blue-700">
        <button
          className="absolute top-3 right-4 text-2xl font-bold text-blue-700 hover:text-red-500 dark:text-blue-200"
          onClick={onClose}
          aria-label="Close dictionary"
        >×</button>
        <h2 className="text-2xl font-extrabold mb-6 text-blue-700 dark:text-blue-200">IT Dictionary</h2>
        <div className="mb-4 text-sm text-gray-500 dark:text-gray-300">
          Tip: Search for any IT term or browse the list below! <br />
          <span className="italic">Press <kbd>Esc</kbd> to close.</span>
        </div>
        <input
          ref={inputRef}
          className="w-full border-2 border-blue-200 dark:border-blue-700 px-4 py-2 mb-6 rounded-lg focus:outline-none focus:border-blue-500 dark:bg-gray-900 dark:text-gray-100"
          placeholder="Search term..."
          value={search}
          onChange={e => setSearch(e.target.value)}
          aria-label="Search IT dictionary"
        />
        <div className="max-h-72 overflow-y-auto">
          {filtered.length === 0 && <div className="text-gray-500 dark:text-gray-300">No terms found.</div>}
          {filtered.map(([term, def]) => (
            <div key={term} className="mb-4">
              <span className="font-semibold text-blue-800 dark:text-blue-200">{term}</span>: <span className="text-gray-700 dark:text-gray-100">{def}</span>
            </div>
          ))}
        </div>
        <div className="mt-4 text-xs text-gray-400 dark:text-gray-500 text-right">
          {filtered.length} of {Object.keys(dictionary).length} terms shown
        </div>
      </div>
    </div>
  );
};

export default Dictionary;
