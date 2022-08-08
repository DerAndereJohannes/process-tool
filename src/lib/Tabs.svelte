<script lang="ts">
  export let items = [];
  export let activeTabValue = 1;
  import {EntityStore} from '../stores';
  import {PluginStore} from '../stores';
  import {MultiStore} from '../stores';


  const handleClick = (tabValue: number) => () => {
    (activeTabValue = tabValue);
    for (let i:number = 0; i < $EntityStore.length; i++) {
        $EntityStore[i].selected = false;
    }

    for (let i:number = 0; i < $PluginStore.length; i++) {
        $PluginStore[i].selected = false;
    }

    if (tabValue === 1 || tabValue === 3) {
        $MultiStore = false;
    } else {
        $MultiStore = true;
    }
  }
</script>

<ul>
{#each items as item}
	<li class={activeTabValue === item.value ? 'active' : ''}>
		<span on:click={handleClick(item.value)}><b>{item.label}</b></span>
	</li>
{/each}
</ul>
{#each items as item}
	{#if activeTabValue == item.value}
	<div class="box">
		<svelte:component this={item.component}/>
	</div>
	{/if}
{/each}
<style>
	.box {
        display: flex;
        flex-direction: column;
		padding: 20px;
		border: 1px solid #dee2e6;
        border-radius: 0 0 .5rem .5rem;
        height: 100%;
		overflow-y: auto;
	}
  ul {
    display: flex;
    flex-wrap: wrap;
    padding-left: 0;
    margin-bottom: 0;
    border-bottom: 1px solid #dee2e6;
  }
	li {
		margin-bottom: -1px;
        list-style-type: none;
		flex-grow: 1;
	}

  span {
    border: 1px solid var(--code-background-color);
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
    display: block;
    padding: 0.5rem 1rem;
    cursor: pointer;
  }

  span:hover {
    border-color: #e9ecef #e9ecef #dee2e6;
  }

  li.active > span {
    color: #495057;
    background-color: #fff;
    border-color: #dee2e6 #dee2e6 #fff;
  }

</style>
