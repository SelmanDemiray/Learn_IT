import React from 'react';
import { Link } from 'react-router-dom';

const Navbar = ({
  onDictionary,
  dark,
  onToggleDark
}: {
  onDictionary: () => void,
  dark?: boolean,
  onToggleDark?: () => void
}) => (
  <nav className="bg-gradient-to-r from-blue-800 to-blue-600 text-white px-6 py-4 flex justify-between items-center shadow-lg">
    <div className="text-2xl font-extrabold tracking-tight drop-shadow font-display">
      IT Beginner Academy
    </div>
    <div className="space-x-6 flex items-center">
      <Link to="/" className="hover:underline">Home</Link>
      <Link to="/lessons" className="hover:underline">Lessons</Link>
      <Link to="/progress" className="hover:underline">Progress</Link>
      <Link to="/skills" className="hover:underline">Skills</Link>
      <button
        className="bg-white text-blue-700 px-3 py-1 rounded hover:bg-blue-100 font-semibold shadow"
        onClick={onDictionary}
        aria-label="Open IT Dictionary"
      >
        Dictionary
      </button>
      {onToggleDark && (
        <button
          className="ml-2 px-2 py-1 rounded bg-gray-200 text-gray-800 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-100 dark:hover:bg-gray-600 transition"
          onClick={onToggleDark}
          aria-label="Toggle dark mode"
        >
          {dark ? '🌙' : '☀️'}
        </button>
      )}
    </div>
  </nav>
);

export default Navbar;
