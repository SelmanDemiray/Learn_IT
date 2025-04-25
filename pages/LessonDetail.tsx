import React from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import lessons from '../data/lessons';

const getProgress = () => {
  const prog = localStorage.getItem('progress');
  return prog ? JSON.parse(prog) : {};
};

const setProgress = (id: string) => {
  const prog = getProgress();
  prog[id] = true;
  localStorage.setItem('progress', JSON.stringify(prog));
};

const LessonDetail = () => {
  const { id } = useParams();
  const lessonIdx = lessons.findIndex(l => l.id === id);
  const lesson = lessons[lessonIdx];
  const navigate = useNavigate();
  const progress = getProgress();

  if (!lesson) return <div>Lesson not found.</div>;

  const handleComplete = () => {
    setProgress(lesson.id);
    navigate('/lessons');
  };

  const prevLesson = lessonIdx > 0 ? lessons[lessonIdx - 1] : null;
  const nextLesson = lessonIdx < lessons.length - 1 ? lessons[lessonIdx + 1] : null;

  return (
    <div>
      <h2 className="text-2xl font-bold mb-4 text-blue-800 dark:text-blue-200">{lesson.title}</h2>
      <div className="mb-8 prose dark:prose-invert">{lesson.content}</div>
      <div className="flex items-center gap-4 mb-8">
        <button
          className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 disabled:opacity-50"
          onClick={handleComplete}
          disabled={progress[lesson.id]}
        >
          {progress[lesson.id] ? 'Completed!' : 'Mark as Complete'}
        </button>
        {prevLesson && (
          <button
            className="bg-blue-200 text-blue-800 px-3 py-1 rounded hover:bg-blue-300"
            onClick={() => navigate(`/lessons/${prevLesson.id}`)}
          >
            ← Previous
          </button>
        )}
        {nextLesson && (
          <button
            className="bg-blue-200 text-blue-800 px-3 py-1 rounded hover:bg-blue-300"
            onClick={() => navigate(`/lessons/${nextLesson.id}`)}
          >
            Next →
          </button>
        )}
      </div>
      <div className="text-sm text-gray-500 dark:text-gray-400">
        <b>Tip:</b> Use the navigation buttons to move between lessons.
      </div>
    </div>
  );
};

export default LessonDetail;
