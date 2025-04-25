import React from 'react';
import { Link } from 'react-router-dom';
import lessons from '../data/lessons';

const skills = [
  { name: 'Computer Basics', desc: 'Understanding how computers work and basic operations.' },
  { name: 'Hardware & Software', desc: 'Learning about physical components and programs.' },
  { name: 'Internet & Web', desc: 'How the internet works and using web browsers.' },
  { name: 'IT Security', desc: 'Keeping information and devices safe.' },
  { name: 'Networking', desc: 'Connecting computers and sharing information.' },
  { name: 'Programming', desc: 'Writing code to make computers perform tasks.' },
  { name: 'Cloud Computing', desc: 'Using online services for storage and computing.' },
  { name: 'Databases', desc: 'Organizing and managing data.' },
  { name: 'Cybersecurity', desc: 'Protecting systems from digital threats.' },
  { name: 'Operating Systems', desc: 'Managing computer hardware and software.' },
  { name: 'Web Development', desc: 'Building websites and web applications.' },
  { name: 'IT Careers', desc: 'Exploring jobs and certifications in IT.' },
  { name: 'Troubleshooting', desc: 'Diagnosing and fixing computer problems.' },
  { name: 'AI & Machine Learning', desc: 'Smart computers that learn from data.' },
  { name: 'Mobile Devices', desc: 'Using smartphones and tablets.' },
  { name: 'Digital Literacy', desc: 'Using technology responsibly and effectively.' },
  { name: 'Productivity Tools', desc: 'Working with word processors, spreadsheets, and more.' },
  { name: 'Data Privacy', desc: 'Protecting your personal information.' },
  { name: 'Ethical Hacking', desc: 'Finding and fixing security weaknesses.' }
];

const lessonMap = Object.fromEntries(
  lessons.map(l => [l.title, l.id])
);

const Skills = () => (
  <div>
    <h2 className="text-3xl font-bold mb-8 text-blue-800 dark:text-blue-200">IT Skills Overview</h2>
    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
      {skills.map(skill => (
        <div key={skill.name} className="bg-white dark:bg-gray-800 border border-blue-100 dark:border-blue-700 rounded-xl p-6 shadow">
          <div className="text-xl font-semibold text-blue-700 dark:text-blue-200 mb-2">{skill.name}</div>
          <div className="text-gray-700 dark:text-gray-200 mb-2">{skill.desc}</div>
          {lessonMap[skill.name] && (
            <Link
              to={`/lessons/${lessonMap[skill.name]}`}
              className="text-blue-600 dark:text-blue-300 hover:underline text-sm"
            >
              Go to lesson →
            </Link>
          )}
        </div>
      ))}
    </div>
  </div>
);

export default Skills;
