(() => {
    const STATE_DURATION = 1000;

    const vanish = () => {
        document.querySelectorAll(".bertrand").forEach((ctr) => {
            ctr.style.display = "none";
        });
    };
    const changeState = (state) => {
        vanish();
        document.querySelector(`.bertrand.bstate\\:${state}`).style
            .display = "block";
    };
    // Make all Bertrand containers disappear
    vanish();
    document.querySelector(".bertrand-init").style.display = "block";

    // Define a queue of the incoming states and pop from there to update what the user sees
    // periodically
    const queue = [];
    setInterval(() => {
        // Pop the first element off the queue (if `null`, pop again)
        let state = queue.shift();
        if (state === null) {
            state = queue.shift();
        }

        if (state) {
            changeState(state);
            // If we've left the queue empty, add an extra `null` which will be removed after the
            // duration, indicating to `onmessage` that it's safe to directly change the state
            // without going through the queue. This ensures we don't get a situation where we
            // send a bunch of consecutive state changes, all delayed appropriately, and then,
            // just as the last one comes in, we get a new one. Without this, that last one would
            // be displayed instantly, and the second-last would barely get displayed.
            if (queue.length === 0) {
                queue.push(null);
            }
        }
    }, STATE_DURATION);

    const selfUrl = window.location.host;
    const ws = new WebSocket(`ws://${selfUrl}/api/ws`);

    ws.onopen = () => {
        console.log("[BERTRAND]: Connected to state arbiter.");
    };

    ws.onmessage = (event) => {
        console.log(`[BERTRAND]: New state: ${event.data}`);
        if (queue.length === 0) {
            changeState(event.data);
            // Make sure this state is displayed for at least the duration
            queue.push(null);
        } else {
            queue.push(event.data);
        }
    };

    ws.onclose = () => {
        console.log("[BERTRAND]: Disconnected from state arbiter.");
    };

    ws.onerror = (error) => {
        console.log(`[BERTRAND]: Error: ${error.data}`);
    };
})();
