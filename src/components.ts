import { JSDOM } from "jsdom";

const page = new JSDOM(
  "<!DOCTYPE html>\n" +
    '<html lang="en">\n' +
    "<head>\n" +
    '  <meta charset="UTF-8">\n' +
    "  <title>Message</title>\n" +
    '  <link rel="stylesheet" href="./index.css">\n' +
    "</head>\n" +
    "<body>\n" +
    '<section class="chat__body">\n' +
    '  <div class="messages">\n' +
    '    <div class="message">\n' +
    '      <div class="message__text">\n' +
    '        <div class="message__text__content">Pretty sweet, right?\n' +
    '          <div class="message__time">18:11</div>\n' +
    "        </div>\n" +
    "      </div>\n" +
    "    </div>\n" +
    '    <div class="message droplet">\n' +
    '      <div class="message__text">\n' +
    '        <div class="message__text__content">Pudding sweet roll gingerbread croissant gummi bears danish biscuit lemon\n' +
    "          drops. Chocolate cake chocolate bar danish soufflé sweet roll toffee caramels tart dessert. Chupa chups\n" +
    "          topping caramels biscuit ice cream chocolate bar. Halvah apple pie chupa chups powder ice cream.\n" +
    '          <div class="message__time">18:12</div>\n' +
    "        </div>\n" +
    "      </div>\n" +
    "    </div>\n" +
    '    <div class="message my-message droplet">\n' +
    '      <div class="message__text">\n' +
    '        <div class="message__text__content">And looks like pudding?\n' +
    '          <div class="message__time">18:12</div>\n' +
    "        </div>\n" +
    "      </div>\n" +
    "    </div>\n" +
    '    <div class="message">\n' +
    '      <div class="message__image"><img class="message__image__content"\n' +
    '                                       src="http://food.fnr.sndimg.com/content/dam/images/food/fullset/2013/12/19/0/FNM_010114-Chia-Seed-Pudding-Recipe_s4x3.jpg.rend.hgtvcom.616.462.jpeg"/>\n' +
    '        <div class="message__time">18:13</div>\n' +
    "      </div>\n" +
    "    </div>\n" +
    '    <div class="message my-message droplet">\n' +
    '      <div class="message__text">\n' +
    '        <div class="message__text__content">Wow! Looks delicious! When we try to eat it?\n' +
    '          <div class="message__time">18:13</div>\n' +
    "        </div>\n" +
    "      </div>\n" +
    "    </div>\n" +
    '    <div class="message my-message droplet">\n' +
    '      <div class="message__image__text"><img class="message__image__content with__caption"\n' +
    '                                             src="https://lh3.googleusercontent.com/bKtCivxtCSwpfupOJo5yMScbAjaRG1t4F4qBI_Xp9J-P1gL6nh9udwO6DaFeYh88=s1920"/>\n' +
    '        <div class="message__text__content">How about donuts with caramelized milk?\n' +
    '          <div class="message__time">18:16</div>\n' +
    "        </div>\n" +
    "      </div>\n" +
    "    </div>\n" +
    '    <div class="message">\n' +
    '      <div class="message__text">\n' +
    '        <div class="message__text__content"> Multiline text\n' +
    "          Multiline text\n" +
    "          Multiline text\n" +
    "          Multiline text\n" +
    "          Multiline text\n" +
    '          <div class="message__time">18:11</div>\n' +
    "        </div>\n" +
    "      </div>\n" +
    "    </div>\n" +
    "  </div>\n" +
    "</section><!-- clip pathes for droplet(?) in the bottom left/right of the message bubble -->\n" +
    '<svg height="0" width="0">\n' +
    "  <defs>\n" +
    '    <clipPath id="left-droplet">\n' +
    '      <path d="M 10,0 A 10,10 0 0 1 0,10 H 16 V 0 Z">\n' +
    "    </clipPath>\n" +
    '    <clipPath id="right-droplet">\n' +
    '      <path d="M 6,0 A 10,10 0 0 0 16,10 H 0 V 0 Z">\n' +
    "    </clipPath>\n" +
    "  </defs>\n" +
    "</svg>\n" +
    "</body>\n" +
    "</html>\n",
  { resources: "usable" }
).window.document;

export default page;
