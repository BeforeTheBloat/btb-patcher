<div class="leftpanel">
    <div class="logo"></div>
    <div class="changelog-box">
        <h3>Changelog</h3>
        <div class="changelog-content">
            <p>1.0.0 - Initial release</p>
            <p>1.1.0 - Added new features</p>
            <p>1.2.0 - Bug fixes and improvements</p>
            <p>1.3.0 - UI enhancements</p>
            <!-- TODO: dynamically get it from github or smth -->
        </div>
    </div>
    <div class="setmode-buttons">
        <div class="button active"> <!-- TODO: make it set to active on click and set default based on current OS -->
            <div class="windows-logo button-icon"></div>
            <span class="button-text">Windows</span>
        </div>
        <div class="button">
            <div class="linux-logo button-icon"></div>
            <span class="button-text">Linux</span>
        </div>
        <div class="button">
            <div class="android-logo button-icon"></div>
            <span class="button-text">Android</span>
        </div>
    </div>
</div>

<style>
    .leftpanel {
        display: flex;
        position: relative;
        flex-direction: column;
        border-radius: 15px;
        width: 30%;
        background: rgba(1, 7, 16, 0.186);
        height: 100%;
        transition: transform 0.3s ease;
        padding: 20px;
        box-sizing: border-box;
    }

    .leftpanel:hover {
        transform: scale(1.02);
    }

    .logo {
        background-image: url(../../image/logo.png);
        background-size: 64px 64px;
        width: 64px;
        height: 64px;
        position: absolute;
        top: 15;
        left: 15px;
    }

    .android-logo {
        background-image: url(../../image/android.png);
        background-size: contain;
        background-repeat: no-repeat;
        background-position: center;
    }

    .windows-logo {
        background-image: url(../../image/windows.png);
        background-size: contain;
        background-repeat: no-repeat;
        background-position: center;
    }

    .linux-logo {
        background-image: url(../../image/linux.png);
        background-size: contain;
        background-repeat: no-repeat;
        background-position: center;
    }

    .setmode-buttons {
        display: flex;
        flex-direction: row;
        gap: 10px;
        width: 100%;
        margin-top: auto;
    }

    .button {
        display: flex;
        flex-direction: column;
        align-items: center;
        background: rgba(7, 7, 32, 0.2);
        border: 1px solid rgba(98, 98, 104, 0.3);
        border-radius: 10px;
        padding: 10px;
        cursor: pointer;
        transition: background 0.3s ease;
        text-align: center;
        width: calc(100% / 3);
    }

    .button:hover {
        background: rgba(34, 41, 84, 0.3);
    }

    .button.active {
        background: rgba(34, 41, 84, 0.3);
        border: 1px solid rgba(98, 98, 104, 0.6);
    }

    .button-icon {
        width: 40px;
        height: 40px;
        margin-bottom: 5px;
    }

    .button-text {
        color: white;
        font-size: 14px;
    }

    .changelog-box {
        flex: 1;
        margin: 70px 0 20px 0;
        padding: 10px;
        background: rgba(7, 7, 32, 0.2);
        border: 1px solid rgba(98, 98, 104, 0.3);
        border-radius: 10px;
        overflow-y: auto;
    }

    .changelog-box h3 {
        margin-top: 0;
        color: white;
    }

    .changelog-content {
        color: white;
        font-size: 14px;
    }

    .changelog-box::-webkit-scrollbar {
        width: 6px;
    }

    .changelog-box::-webkit-scrollbar-track {
        background: rgba(7, 7, 32, 0.2);
        border-radius: 10px;
    }

    .changelog-box::-webkit-scrollbar-thumb {
        background: rgba(98, 98, 104, 0.6);
        border-radius: 10px;
    }

    .changelog-box::-webkit-scrollbar-thumb:hover {
        background: rgba(98, 98, 104, 0.8);
    }
</style>
