<script lang="ts">
    import { createEventDispatcher } from "svelte";

    import { TreeView } from "carbon-components-svelte";

    const dispatch = createEventDispatcher();

    export let cargoes = [];
    export let industries = [];
    export let activeId;

    let tree = [];
    let treeview;

    function UpdateTree() {
        /* Convert cargo to a tree-like structure. */
        let tree_cargo = [];
        for (let cargo of cargoes.sort((a, b) => a.name.localeCompare(b.name))) {
            tree_cargo.push({
                id: cargo.id | 0x1000,
                text: cargo.name,
            });
        }

        /* Convert industry to a tree-like structure. */
        let tree_industry = [];
        for (let industry of industries.sort((a, b) => a.name.localeCompare(b.name))) {
            tree_industry.push({
                id: industry.id | 0x2000,
                text: industry.name,
            });
        }

        tree_cargo.push({
            id: 0x1fff,
            text: "+ New Cargo",
        });
        tree_industry.push({
            id: 0x2fff,
            text: "+ New Industry",
        });

        /* Update the tree. */
        tree = [
            {
                id: 0,
                text: "General",
            },
            {
                id: 1,
                text: "Cargoes",
                children: tree_cargo,
            },
            {
                id: 2,
                text: "Industries",
                children: tree_industry,
            },
        ];
    }

    function TreeSelect(event) {
        /* Expand/collapse top level when clicked. This ensures at most one category is expanded. */
        if (event.detail.id < 10) {
            treeview?.expandNodes((node) => !event.detail.expanded && node.id === event.detail.id);
        }

        let type;
        let id;

        switch (event.detail.id) {
            case 0:
                type = "general";
                id = undefined;
                break;
            case 1:
                type = "cargo";
                id = undefined;
                break;
            case 2:
                type = "industry";
                id = undefined;
                break;

            default:
                type = "none";
                id = undefined;
                break;
        }

        switch (event.detail.id & 0xf000) {
            case 0x1000:
                type = "cargo";
                id = event.detail.id & 0x0fff;
                break;
            case 0x2000:
                type = "industry";
                id = event.detail.id & 0x0fff;
                break;
        }

        dispatch("selected", {
            type,
            id,
        });
    }

    $: if (cargoes || industries) UpdateTree();
</script>

<div class="navigation">
    <div>
        <TreeView bind:this={treeview} bind:activeId hideLabel children={tree} on:select={TreeSelect} />
    </div>
</div>

<style>
    .navigation {
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: auto;
        width: 250px;
    }
</style>
