<script lang="ts">
    import PluginList from './PluginList.svelte';
    import EntityCard from './EntityCard.svelte';
    import { EntityStore, PluginStore, TemplateEntityStore } from '../stores';
    import { invoke } from '@tauri-apps/api/tauri';
    
    let entities;

    
    invoke("get_plugins").then((plugs) => {
        plugs["selected"] = false;
        /* PluginStore.update(n => n.concat(Object(plugs))) */
        PluginStore.update(_ => Object(plugs));
    })

    $: entities = $EntityStore.filter((item: any) => item.selected);
    $: selected = $PluginStore.find((item: any) => item.selected);
    $: entitiesCount = getEntityTypeCount(entities);

    function translateQuantity(quantity:Number|Array<Number>): String {
        if (quantity instanceof Array) {
            if (quantity[1] === Number.MAX_SAFE_INTEGER) {
                return `${quantity[0]} or more instances required.`
            } else {
                return `Between ${quantity[0]} and ${quantity[1]} instances required.`;
            }
        } else {
            return `Exactly ${quantity} instance${quantity === 1? "" : "s"} required.`;
        }
    }

    function getEntityTypeCount(entities: Array<any>): Object {
        let typeCounter: Object = {};
        
        if (entities) {
            for (let i = 0; i < entities.length; i++) {
                if (entities[i].metadata.type in typeCounter) {
                    typeCounter[entities[i].metadata.type] += 1;
                } else {
                    typeCounter[entities[i].metadata.type] = 1;
                }
            }
        }

        return typeCounter;
    }

    const select_all_click = (j: number, multiname: string) => {
        let callerid: string = `${j}:${multiname}`;

        let caller = document.getElementById(callerid) as HTMLInputElement | null;
        let responders = document.querySelectorAll(`[id^="${callerid}"]`);

        if (caller.checked) {
            responders.forEach((ele: HTMLInputElement) => {
                if (ele != caller) {
                    ele.checked = true;
                }
            });
        } else {
            responders.forEach((ele: HTMLInputElement) => {
                if (ele != caller) {
                    ele.checked = false;
                }
            });
        }
        
    }

</script>


<div class="main-screen column-area">
    <div class="no-scroll">
        <h1 class="title-margin">Plugins<hr class="mt-0"></h1>
        <input type="search" id="search" name="search" placeholder="Search">
        <PluginList />
    </div>

    <div class="container no-scroll">
    {#if selected}
        <h1>{selected.name}<hr class="mt-0"></h1>
        <h2>Input<hr class="mt-0"></h2>
        <div class="grid">
        {#each Object.entries(selected.input) as [type, quantity]}
            <div>
                <article>
                <hgroup>
                    <h4>{type}: {entitiesCount[type] ? entitiesCount[type] : 0} Selected</h4>
                    <h5>{translateQuantity(quantity)}</h5>
                </hgroup>
                {#each entities as en}
                    {#if type === en.metadata.type}
                        <EntityCard entity={en} />
                    {/if}
                {/each}
                </article>
            </div>
        {/each}
        </div>
        
        <h2>Parameters<hr class="mt-0"></h2>
        <article>
        <ul>
        {#each selected.parameters as section, j }
            {#if !("shutter" in section)}
                <details open>
                    <summary>{section["header"]}</summary>
                    {#each Object.entries(section) as [param, choices]}
                        {#if param.split(":")[0] == "multichoice"}
                        <fieldset>
                            <legend><b><u>{param.split(":")[1]}</u></b></legend>
                            <label for="{j}:{param}">
                                <input type="checkbox" id="{j}:{param}" name="selectall" on:click={() => select_all_click(j, param)}> Select all
                            </label>
                            {#each choices as choice}
                                <label for="{j}:{param}:{choice}">
                                    <input type="checkbox" id="{j}:{param}:{choice}" name={choice}> {choice}
                                </label>
                            {/each}
                        </fieldset>
                        {/if}
                        
                    {/each}

                </details>

            {:else}
                <details>
                    <summary>{section["header"]}</summary>
                </details>

            {/if}
        {/each}
        </ul>
        </article>
        <h2>Output<hr class="mt-0"></h2>
        <article>
        {#each Object.entries(selected.output) as [type, quantity]}
            <div class="output-format">
                <EntityCard entity={$TemplateEntityStore[type]} /> <h5 class="output-quantity">x{quantity}</h5>
            </div>
        {/each}
        </article>


    {:else}
        
        <h1>Please Select a Plugin...</h1>
    {/if}
    </div>
</div>


<style>
.main-screen {
    display: grid;
    grid-template-columns: fit-content(33%) auto;
    grid-column-gap: 20px;
    justify-items: stretch;
    align-items: stretch;
    height: 100%;
    overflow-y: hidden;
    padding: 0;
    margin: 0;
}

.output-format {
    display: grid;
    grid-template-columns: 90% auto;
    grid-column-gap: 20px;
    justify-items: stretch;
    align-items: stretch;
}

.output-quantity {
    text-align: center;
    margin-bottom: 0;
    line-height: 80px;

}

.no-scroll {
    overflow-y: auto;
}

.title-margin {
    margin-bottom: 10px;
}

</style>
