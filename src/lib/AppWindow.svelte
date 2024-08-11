<div class="window">
    <div class="drag-area" data-tauri-drag-region></div>
    <h1>yooo</h1>
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
</style>