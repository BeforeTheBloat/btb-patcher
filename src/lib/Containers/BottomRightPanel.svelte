<div class="bottomrightpanel">
    <div class="social-buttons">
        <a href="https://discord.gg/CPvKf3KjTE" target="_blank" class="social-button discord"></a>
        <a href="https://www.youtube.com/@BeforeTheBloat" target="_blank" class="social-button youtube"></a>
    </div>
    <div class="launch-button" id="launchButton">Launch</div>
</div>

<script>
    document.addEventListener("DOMContentLoaded", function() {
        const launchButton = document.getElementById("launchButton");

        launchButton.addEventListener("click", async function() {
            if (launchButton.classList.contains("downloading")) return;

            launchButton.classList.add("downloading");
            launchButton.textContent = "Checking...";
            launchButton.style.cursor = "not-allowed";
            launchButton.style.backgroundColor = "#2c9346";

            try {
                await window.__TAURI_INVOKE__("open_minecraft");
                launchButton.textContent = "Minecraft Launched!";
            } catch (error) {
                console.error("Failed to open Minecraft:", error);
                launchButton.textContent = "Failed!";
                alert("An error occurred: " + error.message);
            }

            setTimeout(() => {
                launchButton.classList.remove("downloading");
                launchButton.textContent = "Launch";
                launchButton.style.backgroundColor = "#47C767";
                launchButton.style.cursor = "pointer";
            }, 2000);
        });
    });
</script>
<style>
    .bottomrightpanel {
        position: fixed;
        bottom: 20px;
        right: 20px;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
        width: auto;
        height: auto;
        gap: 10px;
    }

    .social-buttons {
        display: flex;
        background: rgba(7, 7, 32, 0.2);
        border: 1px solid rgba(98, 98, 104, 0.3);
        border-radius: 15px;
        transition: transform 0.3s ease;
    }

    .social-buttons:hover {
        transform: scale(1.05);
    }

    .social-button {
        width: 40px;
        height: 40px;
        cursor: pointer;
        position: relative;
    }

    .social-button:hover {
        background: rgba(34, 41, 84, 0.3);
    }

    .social-button::before {
        content: "";
        position: absolute;
        width: 20px;
        height: 20px;
        transform: translate(-50%, -50%);
        top: 50%;
        left: 50%;
        filter: invert(100%) brightness(100%);
        background-size: contain;
        background-repeat: no-repeat;
        background-position: center;
    }

    .social-button.discord {
        border-top-left-radius: 15px;
        border-bottom-left-radius: 15px;
    }

    .social-button.youtube {
        border-top-right-radius: 15px;
        border-bottom-right-radius: 15px; 
    }

    .social-button.discord::before {
        background-image: url(../../image/discord.png);
    }

    .social-button.youtube::before {
        background-image: url(../../image/youtube.png);
    }

    .launch-button {
        background-color: #47C767;
        color: #16083B;
        border-radius: 15px;
        padding: 10px 20px;
        font-weight: bold;
        font-size: 16px;
        cursor: pointer;
        transition: background-color 0.3s ease, transform 0.3s ease;
    }

    .launch-button:hover {
        background-color: #2c9346;
        transform: scale(1.05);
    }

    .launch-button.downloading {
        background-color: #2c9346;
        cursor: not-allowed;
    }
</style>
