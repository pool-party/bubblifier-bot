/* eslint-disable */
import { Writer, Reader } from 'protobufjs/minimal';

export const protobufPackage = '';

export interface BubbleRequest {
  message: BubbleRequest_Message[];
}

export interface BubbleRequest_Message {
  textMessage: BubbleRequest_Message_TextMessage | undefined;
  photoMessage: BubbleRequest_Message_PhotoMessage | undefined;
  time: string;
  author: boolean;
  authorName: string;
  /** author picture url */
  authorPic: string;
  authorRole: string;
}

export interface BubbleRequest_Message_TextMessage {
  text: string;
}

export interface BubbleRequest_Message_PhotoMessage {
  photoText: string;
  /** photo url */
  photo: string;
}

export interface Response {
  id: string;
}

const baseBubbleRequest: object = {};

export const BubbleRequest = {
  encode(message: BubbleRequest, writer: Writer = Writer.create()): Writer {
    for (const v of message.message) {
      BubbleRequest_Message.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): BubbleRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseBubbleRequest } as BubbleRequest;
    message.message = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.message.push(BubbleRequest_Message.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BubbleRequest {
    const message = { ...baseBubbleRequest } as BubbleRequest;
    message.message = [];
    if (object.message !== undefined && object.message !== null) {
      for (const e of object.message) {
        message.message.push(BubbleRequest_Message.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: BubbleRequest): unknown {
    const obj: any = {};
    if (message.message) {
      obj.message = message.message.map((e) => (e ? BubbleRequest_Message.toJSON(e) : undefined));
    } else {
      obj.message = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<BubbleRequest>): BubbleRequest {
    const message = { ...baseBubbleRequest } as BubbleRequest;
    message.message = [];
    if (object.message !== undefined && object.message !== null) {
      for (const e of object.message) {
        message.message.push(BubbleRequest_Message.fromPartial(e));
      }
    }
    return message;
  },
};

const baseBubbleRequest_Message: object = {
  time: '',
  author: false,
  authorName: '',
  authorPic: '',
  authorRole: '',
};

export const BubbleRequest_Message = {
  encode(message: BubbleRequest_Message, writer: Writer = Writer.create()): Writer {
    if (message.textMessage !== undefined) {
      BubbleRequest_Message_TextMessage.encode(
        message.textMessage,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.photoMessage !== undefined) {
      BubbleRequest_Message_PhotoMessage.encode(
        message.photoMessage,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.time !== '') {
      writer.uint32(26).string(message.time);
    }
    if (message.author === true) {
      writer.uint32(32).bool(message.author);
    }
    if (message.authorName !== '') {
      writer.uint32(42).string(message.authorName);
    }
    if (message.authorPic !== '') {
      writer.uint32(50).string(message.authorPic);
    }
    if (message.authorRole !== '') {
      writer.uint32(58).string(message.authorRole);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): BubbleRequest_Message {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseBubbleRequest_Message } as BubbleRequest_Message;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.textMessage = BubbleRequest_Message_TextMessage.decode(reader, reader.uint32());
          break;
        case 2:
          message.photoMessage = BubbleRequest_Message_PhotoMessage.decode(reader, reader.uint32());
          break;
        case 3:
          message.time = reader.string();
          break;
        case 4:
          message.author = reader.bool();
          break;
        case 5:
          message.authorName = reader.string();
          break;
        case 6:
          message.authorPic = reader.string();
          break;
        case 7:
          message.authorRole = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BubbleRequest_Message {
    const message = { ...baseBubbleRequest_Message } as BubbleRequest_Message;
    if (object.textMessage !== undefined && object.textMessage !== null) {
      message.textMessage = BubbleRequest_Message_TextMessage.fromJSON(object.textMessage);
    } else {
      message.textMessage = undefined;
    }
    if (object.photoMessage !== undefined && object.photoMessage !== null) {
      message.photoMessage = BubbleRequest_Message_PhotoMessage.fromJSON(object.photoMessage);
    } else {
      message.photoMessage = undefined;
    }
    if (object.time !== undefined && object.time !== null) {
      message.time = String(object.time);
    } else {
      message.time = '';
    }
    if (object.author !== undefined && object.author !== null) {
      message.author = Boolean(object.author);
    } else {
      message.author = false;
    }
    if (object.authorName !== undefined && object.authorName !== null) {
      message.authorName = String(object.authorName);
    } else {
      message.authorName = '';
    }
    if (object.authorPic !== undefined && object.authorPic !== null) {
      message.authorPic = String(object.authorPic);
    } else {
      message.authorPic = '';
    }
    if (object.authorRole !== undefined && object.authorRole !== null) {
      message.authorRole = String(object.authorRole);
    } else {
      message.authorRole = '';
    }
    return message;
  },

  toJSON(message: BubbleRequest_Message): unknown {
    const obj: any = {};
    message.textMessage !== undefined &&
      (obj.textMessage = message.textMessage
        ? BubbleRequest_Message_TextMessage.toJSON(message.textMessage)
        : undefined);
    message.photoMessage !== undefined &&
      (obj.photoMessage = message.photoMessage
        ? BubbleRequest_Message_PhotoMessage.toJSON(message.photoMessage)
        : undefined);
    message.time !== undefined && (obj.time = message.time);
    message.author !== undefined && (obj.author = message.author);
    message.authorName !== undefined && (obj.authorName = message.authorName);
    message.authorPic !== undefined && (obj.authorPic = message.authorPic);
    message.authorRole !== undefined && (obj.authorRole = message.authorRole);
    return obj;
  },

  fromPartial(object: DeepPartial<BubbleRequest_Message>): BubbleRequest_Message {
    const message = { ...baseBubbleRequest_Message } as BubbleRequest_Message;
    if (object.textMessage !== undefined && object.textMessage !== null) {
      message.textMessage = BubbleRequest_Message_TextMessage.fromPartial(object.textMessage);
    } else {
      message.textMessage = undefined;
    }
    if (object.photoMessage !== undefined && object.photoMessage !== null) {
      message.photoMessage = BubbleRequest_Message_PhotoMessage.fromPartial(object.photoMessage);
    } else {
      message.photoMessage = undefined;
    }
    if (object.time !== undefined && object.time !== null) {
      message.time = object.time;
    } else {
      message.time = '';
    }
    if (object.author !== undefined && object.author !== null) {
      message.author = object.author;
    } else {
      message.author = false;
    }
    if (object.authorName !== undefined && object.authorName !== null) {
      message.authorName = object.authorName;
    } else {
      message.authorName = '';
    }
    if (object.authorPic !== undefined && object.authorPic !== null) {
      message.authorPic = object.authorPic;
    } else {
      message.authorPic = '';
    }
    if (object.authorRole !== undefined && object.authorRole !== null) {
      message.authorRole = object.authorRole;
    } else {
      message.authorRole = '';
    }
    return message;
  },
};

const baseBubbleRequest_Message_TextMessage: object = { text: '' };

export const BubbleRequest_Message_TextMessage = {
  encode(message: BubbleRequest_Message_TextMessage, writer: Writer = Writer.create()): Writer {
    if (message.text !== '') {
      writer.uint32(10).string(message.text);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): BubbleRequest_Message_TextMessage {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseBubbleRequest_Message_TextMessage,
    } as BubbleRequest_Message_TextMessage;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.text = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BubbleRequest_Message_TextMessage {
    const message = {
      ...baseBubbleRequest_Message_TextMessage,
    } as BubbleRequest_Message_TextMessage;
    if (object.text !== undefined && object.text !== null) {
      message.text = String(object.text);
    } else {
      message.text = '';
    }
    return message;
  },

  toJSON(message: BubbleRequest_Message_TextMessage): unknown {
    const obj: any = {};
    message.text !== undefined && (obj.text = message.text);
    return obj;
  },

  fromPartial(
    object: DeepPartial<BubbleRequest_Message_TextMessage>
  ): BubbleRequest_Message_TextMessage {
    const message = {
      ...baseBubbleRequest_Message_TextMessage,
    } as BubbleRequest_Message_TextMessage;
    if (object.text !== undefined && object.text !== null) {
      message.text = object.text;
    } else {
      message.text = '';
    }
    return message;
  },
};

const baseBubbleRequest_Message_PhotoMessage: object = { photoText: '', photo: '' };

export const BubbleRequest_Message_PhotoMessage = {
  encode(message: BubbleRequest_Message_PhotoMessage, writer: Writer = Writer.create()): Writer {
    if (message.photoText !== '') {
      writer.uint32(10).string(message.photoText);
    }
    if (message.photo !== '') {
      writer.uint32(18).string(message.photo);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): BubbleRequest_Message_PhotoMessage {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseBubbleRequest_Message_PhotoMessage,
    } as BubbleRequest_Message_PhotoMessage;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.photoText = reader.string();
          break;
        case 2:
          message.photo = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BubbleRequest_Message_PhotoMessage {
    const message = {
      ...baseBubbleRequest_Message_PhotoMessage,
    } as BubbleRequest_Message_PhotoMessage;
    if (object.photoText !== undefined && object.photoText !== null) {
      message.photoText = String(object.photoText);
    } else {
      message.photoText = '';
    }
    if (object.photo !== undefined && object.photo !== null) {
      message.photo = String(object.photo);
    } else {
      message.photo = '';
    }
    return message;
  },

  toJSON(message: BubbleRequest_Message_PhotoMessage): unknown {
    const obj: any = {};
    message.photoText !== undefined && (obj.photoText = message.photoText);
    message.photo !== undefined && (obj.photo = message.photo);
    return obj;
  },

  fromPartial(
    object: DeepPartial<BubbleRequest_Message_PhotoMessage>
  ): BubbleRequest_Message_PhotoMessage {
    const message = {
      ...baseBubbleRequest_Message_PhotoMessage,
    } as BubbleRequest_Message_PhotoMessage;
    if (object.photoText !== undefined && object.photoText !== null) {
      message.photoText = object.photoText;
    } else {
      message.photoText = '';
    }
    if (object.photo !== undefined && object.photo !== null) {
      message.photo = object.photo;
    } else {
      message.photo = '';
    }
    return message;
  },
};

const baseResponse: object = { id: '' };

export const Response = {
  encode(message: Response, writer: Writer = Writer.create()): Writer {
    if (message.id !== '') {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Response {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseResponse } as Response;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Response {
    const message = { ...baseResponse } as Response;
    if (object.id !== undefined && object.id !== null) {
      message.id = String(object.id);
    } else {
      message.id = '';
    }
    return message;
  },

  toJSON(message: Response): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial(object: DeepPartial<Response>): Response {
    const message = { ...baseResponse } as Response;
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    } else {
      message.id = '';
    }
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | undefined;
export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;
