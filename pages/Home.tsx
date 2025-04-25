import React from 'react';
import { Link } from 'react-router-dom';

const skills = [
  'Computer Basics',
  'Hardware & Software',
  'Internet & Web',
  'IT Security',
  'Networking',
  'Programming',
  'Cloud Computing',
  'Databases',
  'Cybersecurity',
  'Operating Systems',
  'Web Development',
  'IT Careers',
  'Troubleshooting',
  'AI & Machine Learning',
  'Mobile Devices',
  'Digital Literacy',
  'Productivity Tools',
  'Data Privacy',
  'Ethical Hacking'
];

const featuredTopics = [
  'Operating Systems',
  'Web Development',
  'IT Careers',
  'Troubleshooting',
  'AI & Machine Learning',
  'Mobile Security',
  'Cloud Storage',
  'Digital Citizenship'
];

const testimonials = [
  {
    name: "Alex",
    text: "This site made IT easy and fun! I loved the interactive lessons and dictionary.",
    avatar: "🧑‍💻"
  },
  {
    name: "Maria",
    text: "I started with zero knowledge and now I can help my friends with computers!",
    avatar: "👩‍🎓"
  },
  {
    name: "Sam",
    text: "The progress tracker kept me motivated. Highly recommended for beginners.",
    avatar: "🧑‍🏫"
  }
];

const Home = () => (
  <div className="text-center mt-16">
    <h1 className="text-5xl font-extrabold mb-6 text-blue-800 dark:text-blue-200 drop-shadow-lg font-display">
      Welcome to IT Beginner Academy
    </h1>
    <p className="text-xl mb-8 text-gray-700 dark:text-gray-200">
      Start your journey in IT from absolute zero. Learn step by step, track your progress, and master the basics!
    </p>
    <Link
      to="/lessons"
      className="bg-gradient-to-r from-blue-700 to-blue-500 text-white px-8 py-4 rounded-xl text-xl font-semibold shadow-lg hover:from-blue-800 hover:to-blue-600 transition"
    >
      Start Learning
    </Link>
    <div className="mt-12">
      <h2 className="text-2xl font-bold mb-4 text-blue-700 dark:text-blue-200">What You'll Learn</h2>
      <div className="flex flex-wrap justify-center gap-4">
        {skills.map(skill => (
          <div key={skill} className="bg-white dark:bg-gray-800 border border-blue-200 dark:border-blue-700 rounded-lg px-6 py-3 shadow hover:shadow-lg transition text-lg font-medium text-blue-800 dark:text-blue-200">
            {skill}
          </div>
        ))}
      </div>
    </div>
    <div className="mt-12">
      <h2 className="text-xl font-bold mb-2 text-blue-600 dark:text-blue-300">Featured Topics</h2>
      <div className="flex flex-wrap justify-center gap-3">
        {featuredTopics.map(topic => (
          <span key={topic} className="bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-4 py-2 rounded-full text-base font-semibold shadow">{topic}</span>
        ))}
      </div>
    </div>
    <div className="mt-10">
      <Link
        to="#"
        onClick={() => window.dispatchEvent(new CustomEvent('openDictionary'))}
        className="inline-block bg-yellow-400 text-blue-900 px-6 py-3 rounded-lg font-bold shadow hover:bg-yellow-300 transition"
        aria-label="Explore the IT Dictionary"
      >
        Explore the IT Dictionary
      </Link>
    </div>
    <div className="mt-16">
      <h2 className="text-xl font-bold mb-4 text-blue-700 dark:text-blue-200">What Our Learners Say</h2>
      <div className="flex flex-wrap justify-center gap-6">
        {testimonials.map(t => (
          <div key={t.name} className="bg-white dark:bg-gray-800 border border-blue-100 dark:border-blue-700 rounded-xl px-6 py-4 shadow-xl-blue max-w-xs">
            <div className="text-3xl mb-2">{t.avatar}</div>
            <div className="italic text-gray-700 dark:text-gray-200 mb-2">&quot;{t.text}&quot;</div>
            <div className="font-bold text-blue-700 dark:text-blue-300">{t.name}</div>
          </div>
        ))}
      </div>
    </div>
  </div>
);

export default Home;
