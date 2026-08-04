import { writable } from 'svelte/store'
export interface ProjectInfo { name:string; path:string; url:string; has_index:boolean; has_conn:boolean; has_gitignore:boolean; db_exists?:boolean }
export const projects = writable<ProjectInfo[]>([])
export const selectedProject = writable<string>('')
export const isEmpty = writable<boolean>(true) // first-run empty
