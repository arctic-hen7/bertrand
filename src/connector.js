(() => {
    const vanish = () => {
        document.querySelectorAll(".bertrand").forEach((ctr) => {
            ctr.style.display = "none";
        });
    };
    // Make all Bertrand containers disappear
    vanish();
    document.querySelector(".bertrand-init").style.display = "block";

    const selfUrl = window.location.host;
    const ws = new WebSocket(`ws://${selfUrl}/api/ws`);

    ws.onopen = () => {
        console.log("[BERTRAND]: Connected to state arbiter.");
    };

    ws.onmessage = (event) => {
        console.log(`[BERTRAND]: New state: ${event.data}`);
        vanish();
        document.querySelector(`.bertrand.bstate\\:${event.data}`).style
            .display = "block";
    };

    ws.onclose = () => {
        console.log("[BERTRAND]: Disconnected from state arbiter.");
    };

    ws.onerror = (error) => {
        console.log(`[BERTRAND]: Error: ${error.data}`);
    };
})();
