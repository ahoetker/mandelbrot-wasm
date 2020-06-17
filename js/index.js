
import('../pkg')
    .then(wasm => {
        const allRanges = document.querySelectorAll(".range-wrap");
        allRanges.forEach(wrap => {
            const range = wrap.querySelector(".range");
            const bubble = wrap.querySelector(".bubble");

            range.addEventListener("input", () => {
                setBubble(range, bubble);
            });
            setBubble(range, bubble);
        });

        function setBubble(range, bubble) {
            const val = range.value;
            const min = range.min ? range.min : 0;
            const max = range.max ? range.max : 100;
            const newVal = Number(((val - min) * 100) / (max - min));
            bubble.innerHTML = val;

            // Sorta magic numbers based on size of the native UI thumb
            bubble.style.left = `calc(${newVal}% + (${8 - newVal * 0.15}px))`;
        }
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const nInput = document.getElementById('n');
        const maxStepsInput = document.getElementById('maxSteps');
        const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            const n = parseFloat(nInput.value) || 600;
            const maxSteps = parseFloat(maxStepsInput.value) || 32;
            wasm.draw(canvas, ctx, n, maxSteps);
        });

        wasm.draw(canvas, ctx, 600, 32);
    })
    .catch(console.error);

