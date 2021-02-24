import React from 'react';
import './index.css';

function Message() {
  return (
    <React.Fragment>
      {/* Optional classes: [message-self [droplet [droplet-self]]] */}
      <div className="message" key="first">
        {/* Classes: message__body-text | message__body-image | message__body-caption */}
        <div className="message__body-text">
          {/* Classes: message__content-text | message__content-image [message__content-caption]
            Variants: text, image, image + caption AND text - corresponds to message-body modifier
        */}
          <div className="message__content-text">
            Pretty sweet, right?
            {/* Optional classes: message__time-self (only on message-self!) | message__time-image (only on message__body-image!) */}
            <div className="message__time">18:11</div>
          </div>
        </div>
      </div>
      <div className="message droplet" key="second">
        <div className="message__body-text">
          <div className="message__content-text">
            Pudding sweet roll gingerbread croissant gummi bears danish biscuit lemon drops.
            Chocolate cake chocolate bar danish soufflé sweet roll toffee caramels tart dessert.
            Chupa chups topping caramels biscuit ice cream chocolate bar. Halvah apple pie chupa
            chups powder ice cream.
            <div className="message__time">18:12</div>
          </div>
        </div>
      </div>
      <div className="message message-self droplet droplet-self" key="third">
        <div className="message__body-text">
          <div className="message__content-text">
            And looks like pudding?
            <div className="message__time message__time-self">18:12</div>
          </div>
        </div>
      </div>
      <div className="message" key="fourth">
        <div className="message__body-image">
          <img
            className="message_content-image"
            src="http://food.fnr.sndimg.com/content/dam/images/food/fullset/2013/12/19/0/FNM_010114-Chia-Seed-Pudding-Recipe_s4x3.jpg.rend.hgtvcom.616.462.jpeg"
          />
          <div className="message__time message__time-image">18:13</div>
        </div>
      </div>
      <div className="message message-self droplet droplet-self" key="fifth">
        <div className="message__body-text">
          <div className="message__content-text">
            Wow! Looks delicious! When we try to eat it?
            <div className="message__time message__time-self">18:13</div>
          </div>
        </div>
      </div>
      <div className="message message-self droplet droplet-self" key="sixth">
        <div className="message__body-caption">
          <img
            className="message_content-image message_content-caption"
            src="https://lh3.googleusercontent.com/bKtCivxtCSwpfupOJo5yMScbAjaRG1t4F4qBI_Xp9J-P1gL6nh9udwO6DaFeYh88=s1920"
          />
          <div className="message__content-text">
            How about donuts with caramelized milk?
            <div className="message__time message__time-self">18:16</div>
          </div>
        </div>
      </div>
      <div className="message" key="seventh">
        <div className="message__body-text">
          <div className="message__content-text">
            Multiline text
            <br />
            Multiline text
            <br />
            Multiline text
            <br />
            Multiline text
            <br />
            Multiline text
            <div className="message__time">18:11</div>
          </div>
        </div>
      </div>
    </React.Fragment>
  );
}

export default Message;
