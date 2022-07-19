<script lang="ts">
    import Sigma from "sigma";
    import Graph from "graphology";
    import { random } from "graphology-layout";
    import { parse } from "graphology-gexf/browser";

    export let graph_str: string|null;

    let graph_parse: any;
    /* let camera; */
    /* $: parse_graph(graph_str); */
    
    const parse_graph = (graph: string|null): boolean => {
        if (graph) {
            let container = document.getElementById("viewer_container") as HTMLElement;
            graph_parse = parse(Graph, graph);
            random.assign(graph_parse);
            new Sigma(graph_parse, container, {minCameraRatio: 0.1, maxCameraRatio: 10});
            return true;
            
        } else {
            return false;
        }
    }

</script>

{graph_str}
