'use client'

import {emit, listen} from '@tauri-apps/api/event'

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
const unlisten = await listen('a', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event);
})

// emits the `click` event with the object payload
for (let i = 0; i < 2; i++) {
    emit('global-emit', {
        message: 'Tauri is awesome!',
    })
}
