<script lang="ts">
    /* I tried to make something happen here, it causes insane memory leak, so I have resorted to just displaying the object debug info */
    /* const JSONVIEW: string[] = ["ocel", "table"]; */
    /* const GRAPHVIEW: string[] = ["ocdg", "graph"]; */

    import {EntityStore} from '../stores';
    /* import { invoke } from '@tauri-apps/api/tauri'; */
    /* import JsonViewer from "./JsonViewer.svelte"; */
    import { JsonView } from '@zerodevx/svelte-json-view';
    /* import GraphViewer from "./GraphViewer.svelte"; */
    /* $: entity_str = null; */

    $: entity = $EntityStore.find((item) => item.selected);
    /* $: set_obj_str(entity); */ 

    /* const set_obj_str = (entity: any) => { */
    /*     if (entity) { */
    /*         get_obj_string(entity.metadata["rust-id"]); */
    /*     } */
    /* } */

    /* const get_obj_string = (rust_id: number) => { */
    /*     invoke("get_analysis_view", {rustId: Number(rust_id)}).then((result: string) => {entity_str = result}) */
    /*                                                           .catch((err: string) => {entity_str = err}); */
    /* } */

</script>

{#if entity}
    <JsonView json={entity} depth=1 />
{/if}

<!--
<div id="viewer_container" style="height: 100%; width: 100%;">
{#if entity_str && entity}
    {#if GRAPHVIEW.includes(entity.metadata.type)}
        <GraphViewer graph_str={entity_str}/>
    {:else if JSONVIEW.includes(entity.metadata.type)}
        <JsonViewer json={entity_str} />
    {:else}
        <h3>Analysis View currently not supported for the {entity.metadata.type} type</h3>
    {/if}
{/if}
</div>

-->
