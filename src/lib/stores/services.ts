import { writable } from 'svelte/store'
export const services = writable({ apache:false, mysql:false, apachePort:8080, mysqlPort:3306 })
export type ServiceStatus = { apache:boolean; mysql:boolean; apachePort:number; mysqlPort:number }
