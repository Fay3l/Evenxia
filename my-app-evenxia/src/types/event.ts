export type GetEvent = {
  id: string;
  title: string;
  description: string;
  startDate: string;
  endDate: string;
  address: string;
  created_at: string;
  public: boolean;
  image_url: string;
  category: string;
  user_id: string;
  views: GetEventViews;
  total_places: number;
};

type GetEventViews = {
  views: number;
};

export const events: GetEvent[] = [
  {
    id: "265687-545488-dqs5484",
    title: "Concert Rock",
    description: "Un concert en plein air.",
    startDate: "2025-09-20",
    endDate: "2025-09-21",
    address: "12 Rue des Musiciens, Paris",
    created_at: "2025-08-01T10:00:00Z",
    public: true,
    image_url: "https://example.com/concert.jpg",
    category: "Musique",
    user_id: "user-1",
    views: { views: 150 },
    total_places: 500,
  },
  {
    id: "265687-545488-dqs5484",
    title: "Atelier Peinture",
    description: "Cours de peinture pour débutants.",
    startDate: "2025-09-22",
    endDate: "2025-09-23",
    address: "45 Avenue des Arts, Lyon",
    created_at: "2025-08-02T11:00:00Z",
    public: false,
    image_url: "https://example.com/peinture.jpg",
    category: "Art",
    user_id: "user-2",
    views: { views: 10 },
    total_places: 20,
  },
  {
    id: "265687-545488-dqs5484",
    title: "Salon du Livre",
    description: "Rencontre avec des auteurs.",
    startDate: "2025-09-25",
    endDate: "2025-09-27",
    address: "Parc Expo, Toulouse",
    created_at: "2025-08-03T12:00:00Z",
    public: true,
    image_url: "https://example.com/livre.jpg",
    category: "Littérature",
    user_id: "user-3",
    views: { views: 2140 },
    total_places: 1000,
  },
  {
    id: "265687-545488-dqs5484",
    title: "Course 10km",
    description: "Course en ville pour tous.",
    startDate: "2025-09-28",
    endDate: "2025-09-28",
    address: "Place Centrale, Marseille",
    created_at: "2025-08-04T13:00:00Z",
    public: true,
    image_url: "https://example.com/course.jpg",
    category: "Sport",
    user_id: "user-4",
    views: { views: 34 },
    total_places: 300,
  },
  {
    id: "265687-545488-dqs5484",
    title: "Marché Bio",
    description: "Produits locaux et bio.",
    startDate: "2025-09-30",
    endDate: "2025-09-30",
    address: "Place du Marché, Bordeaux",
    created_at: "2025-08-05T14:00:00Z",
    public: false,
    image_url: "https://example.com/bio.jpg",
    category: "Marché",
    user_id: "user-5",
    views: { views: 41 },
    total_places: 50,
  },
];