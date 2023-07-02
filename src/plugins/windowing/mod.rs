pub use bevy::prelude::*;
use bevy::{window::PrimaryWindow, math::vec2};
use bevy_debug_text_overlay::screen_print;

use crate::settings::SettingsTextRendering;

use super::input::MousePosition;



pub struct WindowingPlugin;


pub enum WindowPanel{
    TextBox{content:String}
}
#[derive(Default)]
pub enum WindowPanelSpawnPosition{
    #[default]
    Center,
    Position(Vec2)
}
pub struct WindowCreationSettings{
    closable:bool,
    movable:bool,
    title:Option<String>,
    width:f32,
    height:f32,
    position:WindowPanelSpawnPosition
}
enum WindowCommands {
    Create(WindowPanel,WindowCreationSettings)
}

#[derive(Resource,Default)]
pub struct WindowManager{
    commands:Vec<WindowCommands>
}




impl WindowManager {

    ///Create a textbox window at the center of the screen
    pub fn createTextBox(&mut self,content:String){
        self.createWithOptions(WindowPanel::TextBox { content }, WindowCreationSettings { closable: true, movable: true, title: None, position: WindowPanelSpawnPosition::Center,width:200.0,height:150.0 })
    }
    pub fn createWithOptions(&mut self,window:WindowPanel,options:WindowCreationSettings){
        self.commands.push(WindowCommands::Create(window, options))
    }
    fn read(&mut self)->Vec<WindowCommands>{
        if self.commands.is_empty(){
            vec![]
        }else{
            let a =std::mem::replace(&mut self.commands, vec![]);
            a
        }
    }
}

impl Plugin for WindowingPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowManager::default())
            .add_systems((window_command_reader,window_dragging,window_drag_set));
    }
}


#[derive(Component,Default)]
pub struct GameWindowPanel{
    being_dragged:bool,
    initial_drag_position_offset:Vec2,
    position:Vec2
}
#[derive(Component)]
pub struct GameWindowPanelMovable;

fn window(commands:&mut Commands,options:WindowCreationSettings,window_width:f32,window_height:f32,settings:&SettingsTextRendering,assets:&AssetServer,content:Entity)->Entity{
    let window_title = options.title
        .map(|title|commands.spawn(TextBundle::from_section(title,TextStyle { font: assets.load(&settings.font_path_bold), font_size: 12.0 * settings.font_scale_multiplier, color: Color::WHITE})).id());

    let mut e = commands.spawn(NodeBundle{
        style:Style {
            display:Display::Flex,
            position_type:PositionType::Absolute,
            flex_direction:FlexDirection::Column,
            min_size:Size::new(Val::Px(options.width),Val::Px(options.height)),
            max_size:Size::width(Val::Px(options.width)),
            // padding:UiRect::all(Val::Px(5.0)),
            position:if let WindowPanelSpawnPosition::Position(p) = options.position{UiRect::new(Val::Px(p.x), Val::Undefined, Val::Px(p.y), Val::Undefined)}else{
                UiRect::new(Val::Px((window_width-options.width)*0.5), Val::Undefined, Val::Px((window_height-options.height)*0.5),Val::Undefined)
            },
            ..Default::default()
         },
         background_color:BackgroundColor(Color::BLACK),
         ..Default::default()
    });
    e.insert(GameWindowPanel{
        position:if let WindowPanelSpawnPosition::Position(p) = options.position{p}else{vec2((window_width-options.width)*0.5, (window_height-options.height)*0.5)},
        ..Default::default()
    });
    
    e.with_children(|parent|{
        parent.spawn(NodeBundle{
            style:Style {
                min_size:Size::new(Val::Px(100.0),Val::Px(15.0)),
                ..Default::default()
            },
            background_color:BackgroundColor(Color::BLUE),
            ..Default::default()
        }).with_children(|parent|{
            parent.spawn(ImageBundle{
                image:UiImage::new(assets.load("close.png")),
                
                style:Style{
                    size:Size::all(Val::Px(10.0)),
                    margin:UiRect::all(Val::Px(2.5)),
                    ..Default::default()
                },
                ..Default::default()
            });
            if let Some(window_title) = window_title {
                parent.spawn(NodeBundle{
                    style:Style{
                        size:Size::width(Val::Percent(100.0)),
                        padding:UiRect::right(Val::Px(15.0)),
                        justify_content:JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                }).add_child(window_title);
            }

        }).insert(GameWindowPanelMovable).insert(Interaction::default());
        parent.spawn(NodeBundle{
            style:Style{
                margin:UiRect::all(Val::Px(5.0)),
                max_size:Size::width(Val::Px(options.width-10.0)),
                ..Default::default()
            },
            ..Default::default()
        }).add_child(content);
    });
    // e.add_child(content);
    e.id()
}
fn text_box(commands:&mut Commands,width:f32,content:String,settings:&SettingsTextRendering,assets:&AssetServer)->Entity{
    commands.spawn(TextBundle{
        text:Text::from_section(content,TextStyle { font: assets.load(&settings.font_path), font_size: 12.0 * settings.font_scale_multiplier, color: Color::WHITE}),
        style:Style{
            max_size:Size::width(Val::Px(width- 10.0)),
            ..Default::default()
        }
        ,..Default::default()
    }).id()
}

fn window_dragging(
    mut windows:Query<(&mut Style,&mut GameWindowPanel)>,
    mouse:Res<MousePosition>,
    input:Res<Input<MouseButton>>
){
    if input.just_released(MouseButton::Left) {
        for (_ , mut window) in windows.iter_mut() {
            if window.being_dragged {
                window.being_dragged = false
            }
        }
    }else{
        for (mut style , mut window) in windows.iter_mut() {
            if !window.being_dragged {continue;}
            let pos = mouse.get_ui() - window.initial_drag_position_offset;
            style.position.left = Val::Px(pos.x);
            style.position.top = Val::Px(pos.y);
            window.position = pos;
        }
    }
}
fn window_drag_set(
    query:Query<(&Parent,&Interaction),(With<GameWindowPanelMovable>,Changed<Interaction>)>,
    mut windows:Query<&mut GameWindowPanel>,
    // input:Res<Input<MouseButton>>
    mouse:Res<MousePosition>
){
    for (parent,interaction) in query.iter() {
        match interaction {
            Interaction::Clicked => {
                //window becomes movable
                let Ok(mut parent) = windows.get_mut(parent.get()) else{continue;};
                parent.being_dragged = true;
                parent.initial_drag_position_offset = mouse.get_ui()-parent.position

            },
            _=>{}
        }
    }
}

fn window_command_reader(
    mut commands:Commands,
    mut manager:ResMut<WindowManager>,
    main_window:Query<&Window,With<PrimaryWindow>>,
    settings:Res<SettingsTextRendering>,
    assets:Res<AssetServer>
){
    let Ok(main_window) = main_window.get_single() else {return;};
    let width = main_window.width();
    let height = main_window.height();
    for command in manager.read().into_iter() {
        match command {
            WindowCommands::Create(panel, options) => {
                // println!("window creation");
                let content = match panel {
                    WindowPanel::TextBox { content } => text_box(&mut commands, options.width,content, &settings, &assets)
                };
                let _ = window(&mut commands, options,width,height,&settings,&assets,content);
            },
        }
    }
}

