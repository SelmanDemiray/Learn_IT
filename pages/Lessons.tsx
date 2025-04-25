import React from 'react';
import { Link } from 'react-router-dom';
import lessons from '../data/lessons';

const getProgress = () => {
  const prog = localStorage.getItem('progress');
  return prog ? JSON.parse(prog) : {};
};

const funLessonIds = [
  'fun-facts', 'build-pc', 'internet-safety-game', 'code-game', 'ai-fun'
];

const badgeLessonIds = {
  'intro': 'New!',
  'security': 'Important',
  'programming': 'Starter Code',
  'ai': 'Trending',
  'webdev': 'Popular'
};

const lessonEmojis: Record<string, string> = {
  'fun-facts': '🤓',
  'build-pc': '🛠️',
  'internet-safety-game': '🕵️',
  'code-game': '💻',
  'ai-fun': '🤖',
  'ai': '🤖',
  'webdev': '🌐',
  'security': '🔒',
  'programming': '👨‍💻',
  'cloud': '☁️',
  'databases': '🗄️',
  'networking': '🌐',
  'os': '💽',
  'careers': '🎓',
  'troubleshooting': '🛠️'
  // ...add more as desired...
};

const Lessons = () => {
  const progress = getProgress();

  return (
    <div>
      <h2 className="text-3xl font-bold mb-8 text-blue-800 dark:text-blue-200">Lessons & Courses</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {[...lessons].sort((a, b) => a.title.localeCompare(b.title)).map((lesson, idx) => (
          <div
            key={lesson.id}
            className={`bg-white dark:bg-gray-800 rounded-xl shadow-md p-6 flex items-center border-2 ${
              progress[lesson.id] ? 'border-green-400' : 'border-blue-100 dark:border-blue-700'
            } hover:shadow-xl transition`}
          >
            <span className="mr-3 text-2xl">
              {lessonEmojis[lesson.id] || '📘'}
            </span>
            <Link
              to={`/lessons/${lesson.id}`}
              className="text-blue-700 dark:text-blue-200 font-semibold hover:underline flex-1 text-lg"
            >
              {idx + 1}. {lesson.title}
            </Link>
            {funLessonIds.includes(lesson.id) && (
              <span className="ml-2 bg-yellow-200 text-yellow-800 px-2 py-1 rounded text-xs font-bold">Recommended for you!</span>
            )}
            {badgeLessonIds[lesson.id] && (
              <span className="ml-2 bg-blue-200 text-blue-800 px-2 py-1 rounded text-xs font-bold">{badgeLessonIds[lesson.id]}</span>
            )}
            {progress[lesson.id] && (
              <span className="ml-2 text-green-600 font-bold">✓ Completed</span>
            )}
          </div>
        ))}
      </div>
      <div className="mt-8 text-sm text-gray-500 dark:text-gray-400">
        <b>Tip:</b> Lessons marked <span className="bg-yellow-200 text-yellow-800 px-2 py-1 rounded text-xs font-bold">Recommended for you!</span> are interactive or fun!
      </div>
    </div>
  );
};

export default Lessons;
