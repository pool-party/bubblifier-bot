import React from 'react';
import './index.css';
import Message from '../Message';

function Section() {
  return (
    <section className="section">
      <div className="section__element">
        <Message />
      </div>
    </section>
  );
}

export default Section;
