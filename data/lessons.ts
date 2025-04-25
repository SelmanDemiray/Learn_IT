const lessons = [
  {
    id: 'intro',
    title: 'What is IT?',
    content: (
      <div>
        <p>
          IT stands for Information Technology. It’s about using computers and software to manage information. IT is everywhere: in businesses, schools, hospitals, and even your phone!
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What computers are and what they do</li>
          <li>What software is</li>
          <li>How IT helps people and businesses</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'hardware',
    title: 'Basic Computer Hardware',
    content: (
      <div>
        <p>
          Hardware is the physical parts of a computer. Examples:
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li><b>CPU</b> (Central Processing Unit): The brain of the computer</li>
          <li><b>RAM</b> (Memory): Short-term memory</li>
          <li><b>Hard Drive/SSD</b>: Long-term storage</li>
          <li><b>Monitor, Keyboard, Mouse</b>: Input/output devices</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'software',
    title: 'What is Software?',
    content: (
      <div>
        <p>
          Software is the programs and apps that run on computers. Examples:
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Operating Systems (Windows, macOS, Linux)</li>
          <li>Web Browsers (Chrome, Firefox)</li>
          <li>Word Processors (Word, Google Docs)</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'internet',
    title: 'The Internet & Web',
    content: (
      <div>
        <p>
          The Internet connects computers worldwide. The Web is a way to access information using browsers.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is a website?</li>
          <li>How to use a browser</li>
          <li>Staying safe online</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'security',
    title: 'Basic IT Security',
    content: (
      <div>
        <p>
          IT security keeps your information safe. Tips:
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Use strong passwords</li>
          <li>Don’t share personal info</li>
          <li>Be careful with emails and links</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'next',
    title: 'Where to Go Next',
    content: (
      <div>
        <p>
          You’ve learned the basics! Next steps:
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Practice using computers</li>
          <li>Explore more advanced topics (coding, networking, etc.)</li>
          <li>Keep learning and experimenting!</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'networking',
    title: 'Introduction to Networking',
    content: (
      <div>
        <p>
          Networking connects computers and devices to share information and resources.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is a network?</li>
          <li>Types: LAN, WAN, Wi-Fi</li>
          <li>Basic network devices: Router, Switch, Modem</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'programming',
    title: 'Introduction to Programming',
    content: (
      <div>
        <p>
          Programming is writing instructions for computers to perform tasks.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is code?</li>
          <li>Popular languages: Python, JavaScript, Java</li>
          <li>Why learn programming?</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'cloud',
    title: 'Cloud Computing Basics',
    content: (
      <div>
        <p>
          Cloud computing means using remote servers on the internet to store, manage, and process data.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is the cloud?</li>
          <li>Examples: Google Drive, Dropbox, AWS</li>
          <li>Benefits: Access anywhere, scalability</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'databases',
    title: 'Databases 101',
    content: (
      <div>
        <p>
          Databases store and organize information for easy access and management.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is a database?</li>
          <li>Types: SQL, NoSQL</li>
          <li>Examples: MySQL, MongoDB</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'cybersecurity',
    title: 'Cybersecurity Essentials',
    content: (
      <div>
        <p>
          Cybersecurity protects computers, networks, and data from unauthorized access and attacks.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Common threats: Viruses, phishing, malware</li>
          <li>How to stay safe: Updates, firewalls, awareness</li>
          <li>Careers in cybersecurity</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'os',
    title: 'Operating Systems Overview',
    content: (
      <div>
        <p>
          An Operating System (OS) manages computer hardware and software resources and provides services for computer programs.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Popular OS: Windows, macOS, Linux</li>
          <li>What does an OS do?</li>
          <li>Basic OS navigation and settings</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'webdev',
    title: 'Web Development Basics',
    content: (
      <div>
        <p>
          Web development is creating websites and web applications for the internet.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is HTML, CSS, JavaScript?</li>
          <li>How websites work</li>
          <li>Building your first web page</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'careers',
    title: 'IT Careers & Certifications',
    content: (
      <div>
        <p>
          IT offers many career paths and certifications to validate your skills.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Popular IT roles: Support, Developer, Analyst, Security</li>
          <li>Certifications: CompTIA, Cisco, AWS, Microsoft</li>
          <li>How to start your IT career</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'troubleshooting',
    title: 'Troubleshooting & Support',
    content: (
      <div>
        <p>
          Troubleshooting is the process of diagnosing and fixing computer problems.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Common issues and solutions</li>
          <li>Basic support tools</li>
          <li>When to ask for help</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'ai',
    title: 'AI & Machine Learning Basics',
    content: (
      <div>
        <p>
          Artificial Intelligence (AI) enables computers to perform tasks that typically require human intelligence.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is AI?</li>
          <li>Examples: voice assistants, image recognition</li>
          <li>Introduction to machine learning</li>
        </ul>
      </div>
    ),
  },
  {
    id: 'fun-facts',
    title: 'Fun Facts About Computers',
    content: (
      <div>
        <p>
          Did you know?
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>The first computer mouse was made of wood!</li>
          <li>TYPEWRITER is the longest word you can type using only the top row of a keyboard.</li>
          <li>The first 1GB hard drive was announced in 1980 and weighed over 500 pounds!</li>
        </ul>
        <div className="mt-4 p-4 bg-yellow-100 rounded">
          <b>Mini-Quiz:</b> <br />
          What is the main function of a mouse?<br />
          <span className="text-gray-500">(A) Store data &nbsp; (B) Input device for pointing &nbsp; (C) Display images</span>
        </div>
      </div>
    ),
  },
  {
    id: 'build-pc',
    title: 'Let\'s Build a PC (Virtually!)',
    content: (
      <div>
        <p>
          Imagine you are building your own computer. What parts do you need?
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>CPU (the brain!)</li>
          <li>Motherboard (the backbone)</li>
          <li>RAM (short-term memory)</li>
          <li>Storage (SSD or HDD)</li>
          <li>Power Supply</li>
          <li>Case</li>
          <li>Monitor, Keyboard, Mouse</li>
        </ul>
        <div className="mt-4 p-4 bg-green-100 rounded">
          <b>Mini-Project:</b> <br />
          Draw your dream PC setup or use an online PC builder to try assembling one!
        </div>
      </div>
    ),
  },
  {
    id: 'internet-safety-game',
    title: 'Internet Safety Adventure',
    content: (
      <div>
        <p>
          You receive an email from an unknown sender with a link. What should you do?
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Never click suspicious links!</li>
          <li>Check the sender’s email address.</li>
          <li>Ask an adult or IT support if unsure.</li>
        </ul>
        <div className="mt-4 p-4 bg-pink-100 rounded">
          <b>Choose Your Path:</b> <br />
          <span className="text-gray-700">A friend sends you a funny video link. What do you check before clicking?</span>
          <ul className="list-disc ml-8 mt-2">
            <li>Is the link from a trusted source?</li>
            <li>Does the message sound like your friend?</li>
            <li>Is the website address familiar?</li>
          </ul>
        </div>
      </div>
    ),
  },
  {
    id: 'code-game',
    title: 'Your First Coding Game!',
    content: (
      <div>
        <p>
          Let’s write a simple code together! Here’s a fun challenge in JavaScript:
        </p>
        <div className="bg-gray-100 rounded p-4 my-2 font-mono text-sm text-left">
          <span className="text-blue-700">// Print your name 5 times</span><br />
          for (let i = 0; i &lt; 5; i++) &#123;<br />
          &nbsp;&nbsp;console.log("Your Name");<br />
          &#125;
        </div>
        <div className="mt-2">
          <b>Try it:</b> Change <code>Your Name</code> to your own name and run it in your browser’s console!
        </div>
        <div className="mt-4 p-4 bg-blue-100 rounded">
          <b>Bonus:</b> Can you make it count from 1 to 5?
        </div>
      </div>
    ),
  },
  {
    id: 'ai-fun',
    title: 'AI in Everyday Life',
    content: (
      <div>
        <p>
          AI is everywhere! Can you spot where you use AI daily?
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Voice assistants (Siri, Alexa, Google Assistant)</li>
          <li>Movie or music recommendations</li>
          <li>Face recognition on your phone</li>
        </ul>
        <div className="mt-4 p-4 bg-purple-100 rounded">
          <b>Fun Activity:</b> <br />
          Ask your voice assistant a fun question, like “Tell me a joke!”<br />
          <span className="text-gray-600">Or try drawing a cat and see if an AI app can recognize it!</span>
        </div>
      </div>
    ),
  },
  {
    id: 'mobile-devices',
    title: 'Mobile Devices & Apps',
    content: (
      <div>
        <p>
          Mobile devices like smartphones and tablets are powerful computers you can carry anywhere.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is a smartphone?</li>
          <li>Popular mobile operating systems: Android, iOS</li>
          <li>Installing and managing apps</li>
          <li>Mobile security tips</li>
        </ul>
        <div className="mt-4 p-4 bg-blue-50 rounded">
          <b>Try it:</b> Explore your phone’s settings and find the “About” section!
        </div>
      </div>
    ),
  },
  {
    id: 'digital-literacy',
    title: 'Digital Literacy & Online Etiquette',
    content: (
      <div>
        <p>
          Digital literacy means knowing how to use technology safely, responsibly, and effectively.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Recognizing trustworthy sources online</li>
          <li>Respectful online communication</li>
          <li>Protecting your digital footprint</li>
        </ul>
        <div className="mt-4 p-4 bg-yellow-50 rounded">
          <b>Discussion:</b> Why is it important to think before you post online?
        </div>
      </div>
    ),
  },
  {
    id: 'productivity-tools',
    title: 'Productivity Tools & Collaboration',
    content: (
      <div>
        <p>
          Productivity tools help you work, learn, and collaborate with others.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>Word processors, spreadsheets, and presentations</li>
          <li>Cloud-based tools: Google Workspace, Microsoft 365</li>
          <li>Collaboration platforms: Slack, Teams, Zoom</li>
        </ul>
        <div className="mt-4 p-4 bg-green-50 rounded">
          <b>Activity:</b> Try creating a shared document with a friend!
        </div>
      </div>
    ),
  },
  {
    id: 'data-privacy',
    title: 'Data Privacy & Protection',
    content: (
      <div>
        <p>
          Data privacy is about keeping your personal information safe and understanding your rights.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is personal data?</li>
          <li>How companies use your data</li>
          <li>Privacy settings and permissions</li>
          <li>Understanding cookies and trackers</li>
        </ul>
        <div className="mt-4 p-4 bg-pink-50 rounded">
          <b>Quiz:</b> What steps can you take to protect your privacy online?
        </div>
      </div>
    ),
  },
  {
    id: 'ethical-hacking',
    title: 'Ethical Hacking & Cyber Awareness',
    content: (
      <div>
        <p>
          Ethical hacking helps organizations find and fix security weaknesses before bad actors do.
        </p>
        <ul className="list-disc ml-6 mt-2">
          <li>What is ethical hacking?</li>
          <li>Common types of cyberattacks</li>
          <li>How to report security issues responsibly</li>
        </ul>
        <div className="mt-4 p-4 bg-red-50 rounded">
          <b>Fun Fact:</b> Some companies pay “bug bounties” for finding security flaws!
        </div>
      </div>
    ),
  },
];

export default lessons;
