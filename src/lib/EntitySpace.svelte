<script>
    import EntityList from './EntityList.svelte'
</script>

<div class="flex-container column-area">
    <div class="flex-item">
        <h1 class="title-margin">Entities<hr class="mt-0"></h1>
    </div>
    <div class="flex-item">
        <EntityList />
    </div>
    <div class="flex-item">
        <hr>
        <button class="contrast outline button-margin">Import</button>
    </div>
</div>

<style>
.flex-container {
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: flex-start;
    align-content: stretch;
    max-height: calc(100vh - 40px);
    min-width: 300px;
    /* align-items: center; */
    }

.flex-item:nth-child(1) {
    order: 0;
    flex: 0 1 auto;
    align-self: auto;
    text-align: center;
    }

.flex-item:nth-child(2) {
    order: 1;
    flex: 1 1 auto;
    align-self: auto;
    overflow-y: auto;
    }

.flex-item:nth-child(3) {
    order: 2;
    flex: 0 1 auto;
    align-self: auto;
    flex-shrink: 0;
}

.button-margin {
    margin-top: 10px;
    margin-bottom: 0px;
}

.title-margin {
    margin-bottom: 10px;
}
</style>
