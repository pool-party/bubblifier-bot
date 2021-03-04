import React from 'react';
import './index.css';
import Messages from '../Messages';

function Section() {
  return (
    <section className="section">
      <div className="section__element">
        <Messages />
      </div>
    </section>
  );
}

export default Section;
