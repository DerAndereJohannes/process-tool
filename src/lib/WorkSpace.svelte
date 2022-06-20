<script>
    import Tab1 from "./Tab1.svelte";
	import Tab2 from "./Tab2.svelte";
	import Tab3 from "./Tab3.svelte";
    import Tabs from "./Tabs.svelte";

    let items = [
        {label: "Overview", value: 1, component: Tab1},
        {label: "Execute", value: 2, component: Tab2},
        {label: "Analyze", value: 3, component: Tab3}
    ]
</script>

<div class="column-area"> 
    <div class="workspace-flex">
        <Tabs {items} />
    </div>
</div>



<style>
    .workspace-flex {
        display: flex;
        flex-direction: column;
        align-items: stretch;
        height: 100%;
    }
</style>
