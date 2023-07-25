import { reactive } from "vue";

type Events = {
  [key: string]: Event;
};
type EventCallback = (value: any) => void;

const events = reactive<Events>({});

export default function useEventBus() {
  function getEventLen() {
    return events.length;
  }

  function emit(eventName: string, data: any = null) {
    const event = events[eventName];

    if (event) {
      event.fire(data);
    }
  }

  function on(eventName: string, callback: EventCallback) {
    let event: Event = events[eventName];
    if (!event) {
      event = new Event(eventName);
      events[eventName] = event;
    }

    event.registerCallback(callback);
  }

  function off(eventName: string, callback: EventCallback) {
    const event = events[eventName];
    if (event && event.callbacks.indexOf(callback) > -1) {
      event.unregisterCallback(callback);
      if (event.callbacks.length === 0) {
        delete events[eventName];
      }
    }
  }

  return {
    emit,
    on,
    off,
  };
}

class Event {
  eventName: string;
  callbacks: EventCallback[];

  constructor(eventName: string) {
    this.eventName = eventName;
    this.callbacks = [];
  }

  registerCallback(callback: EventCallback) {
    this.callbacks.push(callback);
  }

  unregisterCallback(callback: EventCallback) {
    const index = this.callbacks.indexOf(callback);
    if (index > -1) {
      this.callbacks.splice(index, 1);
    }
  }

  fire(data: any) {
    const callbacks = [...this.callbacks];
    callbacks.forEach((callback) => {
      callback(data);
    });
  }
}
