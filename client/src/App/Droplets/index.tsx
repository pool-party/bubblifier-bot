import React from 'react';

function Droplets() {
  return (
    <svg height="0" width="0">
      <defs>
        <clipPath id="left-droplet">
          <path d="M 10,0 A 10,10 0 0 1 0,10 H 16 V 0 Z" />
        </clipPath>
        <clipPath id="right-droplet">
          <path d="M 6,0 A 10,10 0 0 0 16,10 H 0 V 0 Z" />
        </clipPath>
      </defs>
    </svg>
  );
}

export default Droplets;
