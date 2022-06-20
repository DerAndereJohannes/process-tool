import {writable} from 'svelte/store'

// export const SelectedEntityStore = writable([]);

export const MultiStore = writable(false);


export const EntityStore = writable([
    {
        id: 1,
        selected: false,
        metadata: {
            name: "O2C Log",
            type: "ocel",
            "type-long": "Object-Centric Event Log",
            "time-imported": "2022-06-17T10:10:17Z", 
            "file-size": 23893,
            "file-type": "jsonocel"
        },
        instancedata: {
            "rust-id": 1231212,
            "events-count": 123,
            "objects-count": 123
        },
    },
    {
        id: 2,
        selected: false,
        metadata: {
            name: "Order Management Log",
            type: "ocel",
            "type-long": "Object-Centric Event Log",
            "time-imported": "2022-06-17T10:10:17Z", 
            "file-size": 23893,
            "file-type": "jsonocel"
        },
        instancedata: {
            "rust-id": 1244232,
            "events-count": 123,
            "objects-count": 123
        },
    },
    {
        id: 3,
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
        id: 4,
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


export const PluginStore = writable([
    {
        id: 1,
        type: "Generation",
        selected: false,
        name: "Create Object-Centric Directed Graph",
        input: {"ocel": 1},
        output: {"graph": 1},
        parameters: {
            relations: "Object"
        }
    },
    {
        id: 2,
        type: "Combination",
        selected: false,
        name: "Combine Data Tables",
        input: {"table": [2, Number.MAX_SAFE_INTEGER]},
        output: {"table": 1},
        parameters: {"Delete Input": "Boolean"}
    },
    {
        id: 3,
        type: "Extraction",
        selected: false,
        name: "Extract Feature",
        input: {"ocel": 1, "graph": [0, 1]},
        output: {"table": 1},
        parameters: {name: "Object-Type Count"}
    },
    {
        id: 4,
        type: "Generation",
        selected: false,
        name: "Create Object-Centric Directed Graph",
        input: {"ocel": 1},
        output: {"graph": 1},
        parameters: {
            relations: "Object"
        }
    },
    {
        id: 5,
        type: "Combination",
        selected: false,
        name: "Combine Data Tables",
        input: {"table": [2, Number.MAX_SAFE_INTEGER]},
        output: {"table": 1},
        parameters: {"Delete Input": ["Boolean"]}
    },
    {
        id: 6,
        type: "Extraction",
        selected: false,
        name: "Extract Feature",
        input: {"ocel": 1, "graph": [0, 1]},
        output: {"table": 1},
        parameters: {name: "Object-Type Count"}
    }

]);
