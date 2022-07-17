<script lang="ts">
    import {EntityStore} from '../stores';
    import { save } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri';

    const exportfile = () => {
        save().then((path) => {
            invoke("export_entity", { rustId: Number(entity.id), filepath: path }).then((success) => {
            console.log(success + " was exported successfully");

            }).catch((err) => {console.log(err)});
        });
    }

    $: entity = $EntityStore.find((item) => item.selected);
</script>

{#if entity}
<div class="container">
    <div class="title-container">
    <hgroup class="title-item">
        <h2>{entity.metadata.name}</h2>
        <h2>{entity.metadata["type-long"]}</h2>
    </hgroup>
    <div class="title-item">
        <button class="contrast outline" style="width: 100px; float: right;" on:click={exportfile}>Export</button>
    </div>
    </div>
    <hr>


    <h2>Instance Information<hr></h2>
    <article>
        <ul>
        {#each Object.entries(entity.instancedata) as [key, value]}
            <li><b>{key}</b>: {value}</li>
        {/each}
        </ul>
    </article>

    <h2>Metadata Information<hr></h2>
    <article>
        <ul>
        {#each Object.entries(entity.metadata) as [key, value]}
            <li><b>{key}:</b> {value}</li>
        {/each}
        </ul>
    </article>
</div>

{:else}
<div class="container">
    <h1>Welcome to the OCEL Feature Process Tool!<hr></h1>
    <article>
        Tool Introduction.
    </article>
    <h3>How to use this program<hr></h3>
    <article>
        <details>
            <summary>Importing & Exporting Entities</summary>
            <p>boo!</p>
        </details>
        <details>
            <summary>Section: Overview</summary>
            <p>boo!</p>
        </details>
        <details>
            <summary>Section: Plugin</summary>
            <p>boo!</p>
        </details>
        <details>
            <summary>Section: Analyze</summary>
            <p>boo!</p>
        </details>
    </article>
</div>
{/if}


<style>
.title-container {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: flex-start;
    align-content: stretch;
    align-items: stretch;
    }

.title-item:nth-child(1) {
    order: 0;
    flex: 8 1 auto;
    align-self: auto;
    margin-bottom: 0px;
    }

.title-item:nth-child(2) {
    order: 1;
    flex: 1 1 auto;
    align-content: auto;
    } 
</style>
