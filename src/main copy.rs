use macroquad::prelude::*;

const SCREEN_HEIGHT: f32 = 500.;
const SCREEN_WIDTH: f32 = 800.;
const NUMBER_OF_ELEMENTS: usize = 50;
const ELEMENT_SIZE: u32 = (SCREEN_WIDTH / NUMBER_OF_ELEMENTS as f32) as u32;
const ELEMENT_OFFSET: u32 = 5;

#[macroquad::main("TEST!")]
async fn main()
{
    //INIT
    request_new_screen_size(SCREEN_WIDTH,
                            SCREEN_HEIGHT);

    //INITIALISE ARRAY
    let mut list = [0 as u32 ; NUMBER_OF_ELEMENTS];
    for element in list.iter_mut()
    {
        *element = rand::gen_range(10,
                                  SCREEN_HEIGHT as u32 - 10);
    }

    //DRAWING
    bubble_sort(&mut list).await;
}

async fn draw(list: &[u32; NUMBER_OF_ELEMENTS],
              element1: usize,
              element2: usize,
              is_sorted: bool)
{
    let text = get_frame_time().to_string();
    draw_text(&text,
              10.,
              10.,
              8.,
              RED);
    

    clear_background(BEIGE);
    for i in 0..NUMBER_OF_ELEMENTS
    {
        let mut selection_state = BROWN;
        if i == element1 || i == element2
        {
            selection_state = MAROON; 
        }
        draw_rectangle((i * ELEMENT_SIZE as usize) as f32,
                       SCREEN_HEIGHT - list[i] as f32,
                       (ELEMENT_SIZE - ELEMENT_OFFSET) as f32,
                       (list[i]) as f32,
                       selection_state);
                    }
    next_frame().await;
    let mut green_elements = 0;
    while is_sorted
    {
        if green_elements < NUMBER_OF_ELEMENTS
        {
            green_elements += 1;
            draw_green_elements(list, &green_elements).await;
        }
        else
        {
            loop
            {
                draw_green_elements(list, &green_elements).await;
            }
        }
    }
}

async fn draw_green_elements(list: &[u32; NUMBER_OF_ELEMENTS], green_elements: &usize)
{
    clear_background(BEIGE);
    for i in 0..*green_elements
    {
        draw_rectangle((i * ELEMENT_SIZE as usize) as f32,
                      SCREEN_HEIGHT - list[i] as f32,
                      (ELEMENT_SIZE - ELEMENT_OFFSET) as f32,
                      (list[i]) as f32,
                      DARKGREEN);
    }
    next_frame().await
}


async fn bubble_sort(list: &mut[u32; NUMBER_OF_ELEMENTS])
{
    let n = list.len();
    for i in 0..(n - 1)
    {
        for j in 0..(n - 1 - i)
        {
            if list[j as usize] > list[(j+1) as usize]
            {
                list.swap(j, j + 1);
                draw(list, j, j + 1, false).await;
            }
        }
    }
    draw(list, usize::MAX, usize::MAX, true).await
    
}