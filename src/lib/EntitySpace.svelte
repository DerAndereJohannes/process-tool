<script lang=ts>
    import EntityList from './EntityList.svelte';
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from '@tauri-apps/api/tauri';

    import { EntityStore } from '../stores';

    const fileselect = () => {
        let properties = {
            defaultpath: '~/',
            directory: false,
            filters: [{extensions: ['jsonocel', 'gexf'], name: "*"}]
        };
        open(properties).then((path) => {
            /* import file */
            invoke("import_entity", { filepath: path }).then((id) => {
                                                                invoke("get_instance_info", { instanceId: Number(id) }).then((message) => {
                                                                message['id'] = message['metadata']['rust-id']; 
                                                                message['selected'] = false;
                                                                EntityStore.update(n => [...n, Object(message)]);
                                                                console.log(message)})
                                                                                                                       .catch(((err) => console.log(err)));})
                                                       .catch((err) => console.log(err));
            invoke("get_plugins").then((plugs) => {console.log(plugs)});
        });

    };
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
        <button class="contrast outline button-margin" on:click={fileselect}>Import</button>
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
</style>
