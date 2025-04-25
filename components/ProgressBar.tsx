import React from 'react';

const ProgressBar = ({ percent }: { percent: number }) => (
  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded h-5 mb-4 relative overflow-hidden">
    <div
      className="bg-blue-600 dark:bg-blue-400 h-5 rounded transition-all duration-500"
      style={{ width: `${percent}%` }}
    />
    <span className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-xs font-bold text-blue-900 dark:text-blue-100">
      {percent}%
    </span>
  </div>
);

export default ProgressBar;
