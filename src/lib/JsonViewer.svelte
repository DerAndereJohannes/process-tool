<script lang="ts">
    import { JsonView } from '@zerodevx/svelte-json-view';
    export let json: string|null;
    $: json_obj = update_json();

    const update_json = (): object => {
        if (json && json != "na") {
            return JSON.parse(json);
        } else {
            return JSON.parse("{}");
        }
    }

</script>

{#if json_obj}
    <JsonView json={json_obj} depth=1 />
{/if}
