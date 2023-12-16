import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import LayoutContainer from "../layout/LayoutContainer.vue";
import EditorLayout from "../layout/EditorLayout.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "home",
        redirect: "/index",
        component: EditorLayout,
        children: [{
            path: "index",
            component: () => import("../views/HomeView.vue")
        }]
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;