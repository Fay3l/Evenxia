import { GetEvent } from "@/types/event";

const API_URL =  process.env.API_URL || 'http://localhost:5000/api';
console.log("API_URL:", API_URL);
export async function getAllEvents(){
    try {
        const response = await fetch(`${API_URL}/events`);
        return await response.json() ;
    } catch {
        return [] ;
    }
}