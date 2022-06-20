<script>
    import {scale} from 'svelte/transition'
    import {EntityStore} from '../stores'
    import EntityCard from './EntityCard.svelte'
</script>


{#each $EntityStore as es (es.id)}
    <div in:scale>
        <EntityCard entity={es}/>
    </div>
{/each}
