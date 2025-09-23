'use client'
import Card from "@/components/Card";
import { getAllEvents } from "@/fetch/events";
import { useEffect, useState } from 'react';
const API_URL = process.env.API_URL || 'http://localhost:5000/api';

export default function Home() {
  const [events, setEvents] = useState<any>(null);
  useEffect(() => {
    const fetchData = async () => {
      const data = await getAllEvents();
      setEvents(data);
    };
    fetchData();
  }, []);
  console.log(events);
  return (
    <div>
      <div className="flex flex-col gap-4 p-4">
        <div className="navbar bg-base-100 rounded-full  flex justify-between">
          <div className="m-4 font-bold">Evenxia</div>
          <div className="m-4 flex gap-2">
            <button className="btn">Login</button>
            <button className="btn">Sign up</button>
          </div>
        </div>
        <div className="flex items-center justify-center gap-4">
          <input type="text" placeholder="Chercher un événement ..." className="input input-neutral " />
        </div>
        <div className="grid grid-cols-4 gap-6 m-8 max-md:grid grid-cols-1 max-sm:grid-cols-2 max-lg:grid-cols-2">
          {events && events.length > 0 ? events.map((event: any) => (
            <Card
              key={event.id}
              title={event.title}
              description={event.description}
              imageUrl="https://minio-ts.tail8c4493.ts.net/evenxia/pexels-joshsorenson-976866.jpg"
              start_date={event.startDate}
              end_date={event.endDate}
              address={event.address}
              views={event.views} />
          )) : <p>No events found</p>}
        </div>
      </div>
    </div>
  );
}
