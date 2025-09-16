type Event = {
  id: number;
  title: string;
  description: string;
  startDate: string;
  endDate: string;
  address: string;
};

export const events: Event[] = [
  {
    id: 1,
    title: "Concert Rock",
    description: "Un concert en plein air.",
    startDate: "2025-09-20",
    endDate: "2025-09-20",
    address: "12 Rue des Musiciens, Paris",
  },
  {
    id: 2,
    title: "Atelier Peinture",
    description: "Cours de peinture pour débutants.",
    startDate: "2025-09-22",
    endDate: "2025-09-22",
    address: "45 Avenue des Arts, Lyon",
  },
  {
    id: 3,
    title: "Salon du Livre",
    description: "Rencontre avec des auteurs.",
    startDate: "2025-09-25",
    endDate: "2025-09-27",
    address: "Parc Expo, Toulouse",
  },
  {
    id: 4,
    title: "Course 10km",
    description: "Course en ville pour tous.",
    startDate: "2025-09-28",
    endDate: "2025-09-28",
    address: "Place Centrale, Marseille",
  },
  {
    id: 5,
    title: "Marché Bio",
    description: "Produits locaux et bio.",
    startDate: "2025-09-30",
    endDate: "2025-09-30",
    address: "Place du Marché, Bordeaux",
  },
];