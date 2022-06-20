<script lang="ts">
  import EntitySpace from './lib/EntitySpace.svelte'
  import WorkSpace from './lib/WorkSpace.svelte'
</script>

<div class="main-screen">
    <!-- Entity Selection -->
    <EntitySpace />

    <!-- Tabbed Selection -- 1. View Object -- 2. Programs -- -->
    <WorkSpace />
</div>


<style>
.main-screen {
    display: grid;
    grid-template-columns: fit-content(33%) auto;
    grid-column-gap: 20px;
    grid-row-gap: 20px;
    justify-items: stretch;
    align-items: stretch;
    height: 100%;
}
</style>
