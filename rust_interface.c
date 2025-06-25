#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

// Define C equivalents of Rust structs

typedef struct {
    const char *name;
    const char *price;
    const char *url;
    const unsigned int a_n_o_hostel_club;
} Room;

typedef struct {
    const char *name;
    Room *room_options;
    const char *link;
    unsigned int number_of_rooms;
} Hostel;

typedef struct {
    const char *name;
    const char *ano_url;
    Hostel *hostels;
    unsigned int hostels_len;
} City;

typedef struct {
    City *cities;
    unsigned int len;
} ListCCity;

typedef struct {
    const char **strings;
    unsigned int length;
} ListCString;

void print_room(Room *room) {
    printf("\t\tRoom %s for %s\n", room->name, room->price);
}
void print_hostel(Hostel *hostel) {
    printf("\tHostel %s\n", hostel->name);
    for (int i = 0; i < hostel->number_of_rooms; i++) {
        print_room(&hostel->room_options[i]);
    }
}
void print_city(City *city) {
    printf("City %s\n", city->name);
    for (int i = 0; i < city->hostels_len; i++) {
        print_hostel(&city->hostels[i]);
    }
}
void print_city_list(ListCCity *cities) {
    if (!cities)
        printf("city list nao existe\n");

    for (int i = 0; i < cities->len; i++) {
        print_city(&cities->cities[i]);
    }
}

// Declare the Rust function
extern ListCCity *get_many_cities(ListCString cities_names_c,
                                  const char *date_start, const char *date_end);
extern void free_city_list(ListCCity *city);

int main() {
    const char *cities[] = {"Dresden", "Berlin"};
    int num_cities = 2;
    const char *date_start = "2025-07-01";
    const char *date_end = "2025-07-05";

    ListCString cities_names_c;
    cities_names_c.length = num_cities;
    cities_names_c.strings = (const char **)malloc(num_cities * sizeof(char *));

    // Copy city names (ensure null-terminated strings)
    for (int i = 0; i < num_cities; i++) {
        cities_names_c.strings[i] = strdup(cities[i]);
        if (!cities_names_c.strings[i]) {
            fprintf(stderr, "String duplication failed\n");
            // Cleanup
            for (int j = 0; j < i; j++)
                free((void *)cities_names_c.strings[j]);
            free(cities_names_c.strings);
            return 1;
        }
    }

    // Call the Rust function
    ListCCity *result = get_many_cities(cities_names_c, date_start, date_end);

    printf("Concluded the rust part\n");

    print_city_list(result);

    // Cleanup input memory
    for (int i = 0; i < num_cities; i++) {
        free((void *)cities_names_c.strings[i]);
    }

    // Cleanup output memory (requires a Rust-provided free function)
    // Example: extern void free_list_ccity(ListCCity);
    // free_list_ccity(result);

    return 0;
}