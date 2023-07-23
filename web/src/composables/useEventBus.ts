import { reactive } from "vue";

const events = reactive<{ [key: string]: any }>({});

export default function useEventBus() {
  function emit(eventName: string, data: any = null) {
    const event = events[eventName];

    if (event) {
      event.fire(data);
    }
  }

  function on(eventName: string, callback: (value: any) => void) {
    let event: DispatcherEvent = events[eventName];
    if (!event) {
      event = new DispatcherEvent(eventName);
      events[eventName] = event;
    }

    event.registerCallback(callback);
  }

  function off(eventName: string, callback: any) {
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

class DispatcherEvent {
  eventName: string;
  callbacks: { (value: any): void }[];

  constructor(eventName: string) {
    this.eventName = eventName;
    this.callbacks = [];
  }

  registerCallback(callback: (value: any) => void) {
    this.callbacks.push(callback);
  }

  unregisterCallback(callback: (value: any) => void) {
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
