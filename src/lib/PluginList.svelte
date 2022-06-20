<script>
    import {scale} from 'svelte/transition'
    import {PluginStore} from '../stores'
    import PluginCard from './PluginCard.svelte'
</script>


{#each $PluginStore as es (es.id)}
    <div in:scale>
        <PluginCard entity={es}/>
    </div>
{/each}
