<script lang="ts">
    export let entity;
    import {PluginStore} from '../stores';

    const toggleSelected = () => {
        for (let i=0; i<$PluginStore.length; i++) {
            $PluginStore[i].selected = false;
        }
        entity.selected = !entity.selected;
    }
</script>

<div class="card card-flex-container {entity.selected ? 'selected' : ''}" on:click={toggleSelected}> 
    <!-- <div class="card-item"><img class="icon" src="/assets/{icon}" alt="icon"></div> -->
    <div class="card-item">
        <hgroup style="margin-bottom: 0;">
            <b>{entity.name}</b>
            <h5>{entity.type}</h5>
        </hgroup>
    </div>
</div>

<style>
    .card {
        border-radius: var(--border-radius);
        background: var(--card-background-color);
        box-shadow: var(--card-box-shadow);
        padding: calc(var(--block-spacing-vertical)/4) var(--block-spacing-horizontal);
        margin: 5px 1px;
        border: 1px solid white;
        user-select: none;
        -webkit-user-select: none; /* Chrome/Safari */        
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* IE10+ */
    }
.card-flex-container {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: flex-start;
    align-content: stretch;
    align-items: center;
    text-align: left;
    }

.card-item:nth-child(1) {
    order: 0;
    flex: 0 1 auto;
    align-self: auto;
    }

.card-item:nth-child(2) {
    order: 1;
    flex: 1 1 auto;
    align-self: auto;
    margin-left: 20px;
    }

/* .icon { */
/*     height: 50px; */
/*     width: 50px; */
/* } */

.selected {
    border-width: 2px;
    margin: 4px 0px;
}

</style>

