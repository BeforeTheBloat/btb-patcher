<script>
	import BottomRightPanel from "./Containers/BottomRightPanel.svelte";
    import LeftPanel from "./Containers/LeftPanel.svelte";

    function closeWindow() {
        window.close();
    }
</script>

<div class="window">
    <div class="drag-area" data-tauri-drag-region></div>
    <LeftPanel />
    <BottomRightPanel />
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="close-button" on:click={closeWindow} aria-label="Close window">
        <span>×</span>
    </div>
</div>

<style>
    .drag-area {
        position: fixed;
        top: 0;
        left: 0;
        width: calc(100vw - 150px);
        height: 50px;
        z-index: 9999;
    }

    .window {
        background-color: rgba(20, 20, 21, 0.6);
        width: 100vw;
        height: 100vh;
        padding: 16px;
        overflow: hidden;
    }

    .close-button {
        position: absolute;
        top: 10px;
        right: 10px;
        background: rgba(255, 255, 255, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.3);
        color: white;
        border: none;
        border-radius: 10px;
        width: 36px;
        height: 36px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 24px;
        cursor: pointer;
        padding: 0;
        transition: background 0.3s ease;
        z-index: 10000;
    }

    .close-button span {
        margin-bottom: 1px;
    }

    .close-button:hover {
        background: rgba(255, 0, 0, 0.15);
    }
</style>