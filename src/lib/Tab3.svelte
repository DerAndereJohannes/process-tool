<script lang="ts">
    const JSONVIEW: string[] = ["ocel", "table"];
    const GRAPHVIEW: string[] = ["ocdg", "graph"];

    import {EntityStore} from '../stores';
    import JsonViewer from "./JsonViewer.svelte";
    import GraphViewer from "./GraphViewer.svelte";

    $: entity = $EntityStore.find((item) => item.selected);

</script>

{#if entity}
    {#if JSONVIEW.includes(entity.metadata.type)}
        json my guy
    {:else}
        graph my guy
    {/if}
{/if}
