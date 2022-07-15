import {writable} from 'svelte/store'

// export const SelectedEntityStore = writable([]);

export const MultiStore = writable(false);


export const EntityStore = writable([]);


export const TemplateEntityStore = writable({
    ocel: {
        metadata: {
            name: "New OCEL Entity",
            type: "ocel"
        }
    },
    ocdg: {
        metadata: {
            name: "Object-Centric Directed Graph",
            type: "ocdg"
        }
    },

    graph: {
        metadata: {
            name: "New Graph Entity",
            type: "graph"
        }
    },
    table: {
        metadata: {
            name: "New Table Entity",
            type: "table"
        }
    }
});


export const PluginStore = writable([]);
