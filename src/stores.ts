import {writable} from 'svelte/store'

// export const SelectedEntityStore = writable([]);

export const MultiStore = writable(false);


export const EntityStore = writable([
        {
        id: 3123123,
        selected: false,
        metadata: {
            name: "Directed Graph 1",
            type: "graph",
            "type-long": "Graph",
            "time-imported": "2022-06-17T10:10:17Z", 
            "file-size": 23893,
            "file-type": "cache"
        },
        instancedata: {
            "rust-id": 3435234,
            "nodes-count": 123,
            "edges-count": 12300
        },
    },
    {
        id: 4123123,
        selected: false,
        metadata: {
            name: "Data Table 1",
            type: "table",
            "type-long": "Data Table",
            "time-imported": "2022-06-17T10:10:17Z", 
            "file-size": 23893,
            "file-type": "cache"
        },
        instancedata: {
            "rust-id": 234234,
            "rows": 123,
            "columns": 123
        },
    }
]);


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
