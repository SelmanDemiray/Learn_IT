import React, { useState } from 'react';
import lessons from '../data/lessons';
import ProgressBar from '../components/ProgressBar';

const getProgress = () => {
  const prog = localStorage.getItem('progress');
  return prog ? JSON.parse(prog) : {};
};

const Progress = () => {
  const [_, setRerender] = useState(0);
  const progress = getProgress();
  const completed = lessons.filter(l => progress[l.id]).length;
  const percent = Math.round((completed / lessons.length) * 100);

  const handleReset = () => {
    localStorage.removeItem('progress');
    setRerender(x => x + 1);
  };

  return (
    <div>
      <h2 className="text-3xl font-bold mb-4 text-blue-800 dark:text-blue-200">Your Progress</h2>
      <ProgressBar percent={percent} />
      <p className="mb-4 text-lg">{completed} of {lessons.length} lessons completed.</p>
      <ul>
        {lessons.map(lesson => (
          <li key={lesson.id} className="mb-2 flex items-center">
            <span className={progress[lesson.id] ? 'text-green-600 font-semibold' : 'text-gray-700 dark:text-gray-200'}>
              {lesson.title}
            </span>
            {progress[lesson.id] && <span className="ml-2 text-green-600">✓</span>}
          </li>
        ))}
      </ul>
      <div className="mt-6 flex items-center gap-4">
        {percent === 100 && (
          <span className="bg-green-200 text-green-800 px-3 py-1 rounded font-bold">🏅 All Lessons Complete!</span>
        )}
        <button
          className="bg-red-100 text-red-700 px-3 py-1 rounded hover:bg-red-200 text-sm"
          onClick={handleReset}
        >
          Reset Progress
        </button>
      </div>
    </div>
  );
};

export default Progress;
