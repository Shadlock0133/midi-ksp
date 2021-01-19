/// Main kRPC service, used by clients to interact with basic server
/// functionality.
mod KRPC {
    /// A server side expression.
    struct Expression;

    /// A server side expression.
    struct Type;

    /// The game scene. See KRPC.CurrentGameScene.
    enum GameScene {
        /// The game scene showing the Kerbal Space Center buildings.
        SpaceCenter = 0,
        /// The game scene showing a vessel in flight (or on the
        /// launchpad/runway).
        Flight = 1,
        /// The tracking station.
        TrackingStation = 2,
        /// The Vehicle Assembly Building.
        EditorVAB = 3,
        /// The Space Plane Hangar.
        EditorSPH = 4,
    }

    /// Returns the identifier for the current client.
    fn GetClientID() -> Bytes;

    /// Returns the name of the current client.
    /// This is an empty string if the client has no name.
    fn GetClientName() -> String;

    /// Returns some information about the server, such as the version.
    fn GetStatus() -> Status;

    /// Returns information on all services, procedures, classes, properties
    /// etc. provided by the server.
    /// Can be used by client libraries to automatically create functionality
    /// such as stubs.
    fn GetServices() -> Services;

    /// Add a streaming request and return its identifier.
    fn AddStream(call: ProcedureCall, start: Bool) -> Stream;

    /// Start a previously added streaming request.
    fn StartStream(id: Uint64);

    /// Set the update rate for a stream in Hz.
    fn SetStreamRate(id: Uint64, rate: Float);

    /// Remove a streaming request.
    fn RemoveStream(id: Uint64);

    /// Create an event from a server side expression.
    fn AddEvent(expression: Class) -> Event;

    /// A list of RPC clients that are currently connected to the server.
    /// Each entry in the list is a clients identifier, name and address.
    fn get_Clients() -> List;

    /// Get the current game scene.
    fn get_CurrentGameScene() -> Enumeration;

    /// Whether the game is paused.
    fn get_Paused() -> Bool;

    /// Whether the game is paused.
    fn set_Paused(value: Bool);

    /// A constant value of double precision floating point type.
    /// 
    /// <param name="value"></param>
    fn Expression_static_ConstantDouble(value: Double) -> Class;

    /// A constant value of single precision floating point type.
    /// 
    /// <param name="value"></param>
    fn Expression_static_ConstantFloat(value: Float) -> Class;

    /// A constant value of integer type.
    /// 
    /// <param name="value"></param>
    fn Expression_static_ConstantInt(value: Sint32) -> Class;

    /// A constant value of boolean type.
    /// 
    /// <param name="value"></param>
    fn Expression_static_ConstantBool(value: Bool) -> Class;

    /// A constant value of string type.
    /// 
    /// <param name="value"></param>
    fn Expression_static_ConstantString(value: String) -> Class;

    /// An RPC call.
    /// 
    /// <param name="call"></param>
    fn Expression_static_Call(call: ProcedureCall) -> Class;

    /// Equality comparison.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_Equal(arg0: Class, arg1: Class) -> Class;

    /// Inequality comparison.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_NotEqual(arg0: Class, arg1: Class) -> Class;

    /// Greater than numerical comparison.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_GreaterThan(arg0: Class, arg1: Class) -> Class;

    /// Greater than or equal numerical comparison.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_GreaterThanOrEqual(arg0: Class, arg1: Class) -> Class;

    /// Less than numerical comparison.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_LessThan(arg0: Class, arg1: Class) -> Class;

    /// Less than or equal numerical comparison.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_LessThanOrEqual(arg0: Class, arg1: Class) -> Class;

    /// Boolean and operator.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_And(arg0: Class, arg1: Class) -> Class;

    /// Boolean or operator.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_Or(arg0: Class, arg1: Class) -> Class;

    /// Boolean exclusive-or operator.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_ExclusiveOr(arg0: Class, arg1: Class) -> Class;

    /// Boolean negation operator.
    /// 
    /// <param name="arg"></param>
    fn Expression_static_Not(arg: Class) -> Class;

    /// Numerical addition.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_Add(arg0: Class, arg1: Class) -> Class;

    /// Numerical subtraction.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_Subtract(arg0: Class, arg1: Class) -> Class;

    /// Numerical multiplication.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_Multiply(arg0: Class, arg1: Class) -> Class;

    /// Numerical division.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_Divide(arg0: Class, arg1: Class) -> Class;

    /// Numerical modulo operator.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    /// # Returns
    /// 
    /// The remainder of arg0 divided by arg1
    fn Expression_static_Modulo(arg0: Class, arg1: Class) -> Class;

    /// Numerical power operator.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    /// # Returns
    /// 
    /// arg0 raised to the power of arg1, with type of arg0
    fn Expression_static_Power(arg0: Class, arg1: Class) -> Class;

    /// Bitwise left shift.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_LeftShift(arg0: Class, arg1: Class) -> Class;

    /// Bitwise right shift.
    /// 
    /// <param name="arg0"></param>
    /// <param name="arg1"></param>
    fn Expression_static_RightShift(arg0: Class, arg1: Class) -> Class;

    /// Perform a cast to the given type.
    /// 
    /// <param name="arg"></param>
    /// <param name="type">Type to cast the argument to.</param>
    fn Expression_static_Cast(arg: Class, r#type: Class) -> Class;

    /// A named parameter of type double.
    /// 
    /// # Returns
    /// 
    /// A named parameter.
    /// 
    /// <param name="name">The name of the parameter.</param>
    /// <param name="type">The type of the parameter.</param>
    fn Expression_static_Parameter(name: String, r#type: Class) -> Class;

    /// A function.
    /// 
    /// # Returns
    /// 
    /// A function.
    /// 
    /// <param name="parameters">The parameters of the function.</param>
    /// <param name="body">The body of the function.</param>
    fn Expression_static_Function(parameters: List, body: Class) -> Class;

    /// A function call.
    /// 
    /// # Returns
    /// 
    /// A function call.
    /// 
    /// <param name="function">The function to call.</param>
    /// <param name="args">The arguments to call the function with.</param>
    fn Expression_static_Invoke(function: Class, args: Dictionary) -> Class;

    /// Construct a tuple.
    /// 
    /// # Returns
    /// 
    /// The tuple.
    /// 
    /// <param name="elements">The elements.</param>
    fn Expression_static_CreateTuple(elements: List) -> Class;

    /// Construct a list.
    /// 
    /// # Returns
    /// 
    /// The list.
    /// 
    /// <param name="values">The value. Should all be of the same type.</param>
    fn Expression_static_CreateList(values: List) -> Class;

    /// Construct a set.
    /// 
    /// # Returns
    /// 
    /// The set.
    /// 
    /// <param name="values">The values. Should all be of the same type.</param>
    fn Expression_static_CreateSet(values: Set) -> Class;

    /// Construct a dictionary, from a list of corresponding keys and values.
    /// 
    /// # Returns
    /// 
    /// The dictionary.
    /// 
    /// <param name="keys">The keys. Should all be of the same type.</param>
    /// <param name="values">The values. Should all be of the same type.</param>
    fn Expression_static_CreateDictionary(keys: List, values: List) -> Class;

    /// Convert a collection to a list.
    /// 
    /// # Returns
    /// 
    /// The collection as a list.
    /// 
    /// <param name="arg">The collection.</param>
    fn Expression_static_ToList(arg: Class) -> Class;

    /// Convert a collection to a set.
    /// 
    /// # Returns
    /// 
    /// The collection as a set.
    /// 
    /// <param name="arg">The collection.</param>
    fn Expression_static_ToSet(arg: Class) -> Class;

    /// Access an element in a tuple, list or dictionary.
    /// 
    /// # Returns
    /// 
    /// The element.
    /// 
    /// <param name="arg">The tuple, list or dictionary.</param>
    /// <param name="index">The index of the element to access.
    /// A zero indexed integer for a tuple or list, or a key for a
    /// dictionary.</param>
    fn Expression_static_Get(arg: Class, index: Class) -> Class;

    /// Number of elements in a collection.
    /// 
    /// # Returns
    /// 
    /// The number of elements in the collection.
    /// 
    /// <param name="arg">The list, set or dictionary.</param>
    fn Expression_static_Count(arg: Class) -> Class;

    /// Sum all elements of a collection.
    /// 
    /// # Returns
    /// 
    /// The sum of the elements in the collection.
    /// 
    /// <param name="arg">The list or set.</param>
    fn Expression_static_Sum(arg: Class) -> Class;

    /// Maximum of all elements in a collection.
    /// 
    /// # Returns
    /// 
    /// The maximum elements in the collection.
    /// 
    /// <param name="arg">The list or set.</param>
    fn Expression_static_Max(arg: Class) -> Class;

    /// Minimum of all elements in a collection.
    /// 
    /// # Returns
    /// 
    /// The minimum elements in the collection.
    /// 
    /// <param name="arg">The list or set.</param>
    fn Expression_static_Min(arg: Class) -> Class;

    /// Minimum of all elements in a collection.
    /// 
    /// # Returns
    /// 
    /// The minimum elements in the collection.
    /// 
    /// <param name="arg">The list or set.</param>
    fn Expression_static_Average(arg: Class) -> Class;

    /// Run a function on every element in the collection.
    /// 
    /// # Returns
    /// 
    /// The modified collection.
    /// 
    /// <param name="arg">The list or set.</param>
    /// <param name="func">The function.</param>
    fn Expression_static_Select(arg: Class, func: Class) -> Class;

    /// Run a function on every element in the collection.
    /// 
    /// # Returns
    /// 
    /// The modified collection.
    /// 
    /// <param name="arg">The list or set.</param>
    /// <param name="func">The function.</param>
    fn Expression_static_Where(arg: Class, func: Class) -> Class;

    /// Determine if a collection contains a value.
    /// 
    /// # Returns
    /// 
    /// Whether the collection contains a value.
    /// 
    /// <param name="arg">The collection.</param>
    /// <param name="value">The value to look for.</param>
    fn Expression_static_Contains(arg: Class, value: Class) -> Class;

    /// Applies an accumulator function over a sequence.
    /// 
    /// # Returns
    /// 
    /// The accumulated value.
    /// 
    /// <param name="arg">The collection.</param>
    /// <param name="func">The accumulator function.</param>
    fn Expression_static_Aggregate(arg: Class, func: Class) -> Class;

    /// Applies an accumulator function over a sequence, with a given seed.
    /// 
    /// # Returns
    /// 
    /// The accumulated value.
    /// 
    /// <param name="arg">The collection.</param>
    /// <param name="seed">The seed value.</param>
    /// <param name="func">The accumulator function.</param>
    fn Expression_static_AggregateWithSeed(arg: Class, seed: Class, func: Class) -> Class;

    /// Concatenate two sequences.
    /// 
    /// # Returns
    /// 
    /// The first sequence followed by the second sequence.
    /// 
    /// <param name="arg1">The first sequence.</param>
    /// <param name="arg2">The second sequence.</param>
    fn Expression_static_Concat(arg1: Class, arg2: Class) -> Class;

    /// Order a collection using a key function.
    /// 
    /// # Returns
    /// 
    /// The ordered collection.
    /// 
    /// <param name="arg">The collection to order.</param>
    /// <param name="key">A function that takes a value from the collection and
    /// generates a key to sort on.</param>
    fn Expression_static_OrderBy(arg: Class, key: Class) -> Class;

    /// Determine whether all items in a collection satisfy a boolean predicate.
    /// 
    /// # Returns
    /// 
    /// Whether all items satisfy the predicate.
    /// 
    /// <param name="arg">The collection.</param>
    /// <param name="predicate">The predicate function.</param>
    fn Expression_static_All(arg: Class, predicate: Class) -> Class;

    /// Determine whether any item in a collection satisfies a boolean
    /// predicate.
    /// 
    /// # Returns
    /// 
    /// Whether any item satisfies the predicate.
    /// 
    /// <param name="arg">The collection.</param>
    /// <param name="predicate">The predicate function.</param>
    fn Expression_static_Any(arg: Class, predicate: Class) -> Class;

    /// Double type.
    fn Type_static_Double() -> Class;

    /// Float type.
    fn Type_static_Float() -> Class;

    /// Int type.
    fn Type_static_Int() -> Class;

    /// Bool type.
    fn Type_static_Bool() -> Class;

    /// String type.
    fn Type_static_String() -> Class;

}

/// Provides functionality for drawing objects in the flight scene.
/// 
/// <remarks>
/// For drawing and interacting with the user interface, see the UI service.
/// </remarks>
mod Drawing {
    /// A line. Created using Drawing.AddLine.
    struct Line;

    /// A polygon. Created using Drawing.AddPolygon.
    struct Polygon;

    /// Text. Created using Drawing.AddText.
    struct Text;

    /// Draw a line in the scene.
    /// 
    /// <param name="start">Position of the start of the line.</param>
    /// <param name="end">Position of the end of the line.</param>
    /// <param name="referenceFrame">Reference frame that the positions are
    /// in.</param>
    /// <param name="visible">Whether the line is visible.</param>
    fn AddLine(start: Tuple, end: Tuple, referenceFrame: Class, visible: Bool) -> Class;

    /// Draw a direction vector in the scene, from the center of mass of the
    /// active vessel.
    /// 
    /// <param name="direction">Direction to draw the line in.</param>
    /// <param name="referenceFrame">Reference frame that the direction is
    /// in.</param>
    /// <param name="length">The length of the line.</param>
    /// <param name="visible">Whether the line is visible.</param>
    fn AddDirection(direction: Tuple, referenceFrame: Class, length: Float, visible: Bool) -> Class;

    /// Draw a polygon in the scene, defined by a list of vertices.
    /// 
    /// <param name="vertices">Vertices of the polygon.</param>
    /// <param name="referenceFrame">Reference frame that the vertices are
    /// in.</param>
    /// <param name="visible">Whether the polygon is visible.</param>
    fn AddPolygon(vertices: List, referenceFrame: Class, visible: Bool) -> Class;

    /// Draw text in the scene.
    /// 
    /// <param name="text">The string to draw.</param>
    /// <param name="referenceFrame">Reference frame that the text position is
    /// in.</param>
    /// <param name="position">Position of the text.</param>
    /// <param name="rotation">Rotation of the text, as a quaternion.</param>
    /// <param name="visible">Whether the text is visible.</param>
    fn AddText(text: String, referenceFrame: Class, position: Tuple, rotation: Tuple, visible: Bool) -> Class;

    /// Remove all objects being drawn.
    /// 
    /// <param name="clientOnly">If true, only remove objects created by the
    /// calling client.</param>
    fn Clear(clientOnly: Bool);

    /// Remove the object.
    fn Line_Remove(this: Class);

    /// Start position of the line.
    fn Line_get_Start(this: Class) -> Tuple;

    /// Start position of the line.
    fn Line_set_Start(this: Class, value: Tuple);

    /// End position of the line.
    fn Line_get_End(this: Class) -> Tuple;

    /// End position of the line.
    fn Line_set_End(this: Class, value: Tuple);

    /// Set the color
    fn Line_get_Color(this: Class) -> Tuple;

    /// Set the color
    fn Line_set_Color(this: Class, value: Tuple);

    /// Set the thickness
    fn Line_get_Thickness(this: Class) -> Float;

    /// Set the thickness
    fn Line_set_Thickness(this: Class, value: Float);

    /// Reference frame for the positions of the object.
    fn Line_get_ReferenceFrame(this: Class) -> Class;

    /// Reference frame for the positions of the object.
    fn Line_set_ReferenceFrame(this: Class, value: Class);

    /// Whether the object is visible.
    fn Line_get_Visible(this: Class) -> Bool;

    /// Whether the object is visible.
    fn Line_set_Visible(this: Class, value: Bool);

    /// Material used to render the object.
    /// Creates the material from a shader with the given name.
    fn Line_get_Material(this: Class) -> String;

    /// Material used to render the object.
    /// Creates the material from a shader with the given name.
    fn Line_set_Material(this: Class, value: String);

    /// Remove the object.
    fn Polygon_Remove(this: Class);

    /// Vertices for the polygon.
    fn Polygon_get_Vertices(this: Class) -> List;

    /// Vertices for the polygon.
    fn Polygon_set_Vertices(this: Class, value: List);

    /// Set the color
    fn Polygon_get_Color(this: Class) -> Tuple;

    /// Set the color
    fn Polygon_set_Color(this: Class, value: Tuple);

    /// Set the thickness
    fn Polygon_get_Thickness(this: Class) -> Float;

    /// Set the thickness
    fn Polygon_set_Thickness(this: Class, value: Float);

    /// Reference frame for the positions of the object.
    fn Polygon_get_ReferenceFrame(this: Class) -> Class;

    /// Reference frame for the positions of the object.
    fn Polygon_set_ReferenceFrame(this: Class, value: Class);

    /// Whether the object is visible.
    fn Polygon_get_Visible(this: Class) -> Bool;

    /// Whether the object is visible.
    fn Polygon_set_Visible(this: Class, value: Bool);

    /// Material used to render the object.
    /// Creates the material from a shader with the given name.
    fn Polygon_get_Material(this: Class) -> String;

    /// Material used to render the object.
    /// Creates the material from a shader with the given name.
    fn Polygon_set_Material(this: Class, value: String);

    /// A list of all available fonts.
    fn Text_static_AvailableFonts() -> List;

    /// Remove the object.
    fn Text_Remove(this: Class);

    /// Position of the text.
    fn Text_get_Position(this: Class) -> Tuple;

    /// Position of the text.
    fn Text_set_Position(this: Class, value: Tuple);

    /// Rotation of the text as a quaternion.
    fn Text_get_Rotation(this: Class) -> Tuple;

    /// Rotation of the text as a quaternion.
    fn Text_set_Rotation(this: Class, value: Tuple);

    /// The text string
    fn Text_get_Content(this: Class) -> String;

    /// The text string
    fn Text_set_Content(this: Class, value: String);

    /// Name of the font
    fn Text_get_Font(this: Class) -> String;

    /// Name of the font
    fn Text_set_Font(this: Class, value: String);

    /// Font size.
    fn Text_get_Size(this: Class) -> Sint32;

    /// Font size.
    fn Text_set_Size(this: Class, value: Sint32);

    /// Character size.
    fn Text_get_CharacterSize(this: Class) -> Float;

    /// Character size.
    fn Text_set_CharacterSize(this: Class, value: Float);

    /// Font style.
    fn Text_get_Style(this: Class) -> Enumeration;

    /// Font style.
    fn Text_set_Style(this: Class, value: Enumeration);

    /// Alignment.
    fn Text_get_Alignment(this: Class) -> Enumeration;

    /// Alignment.
    fn Text_set_Alignment(this: Class, value: Enumeration);

    /// Line spacing.
    fn Text_get_LineSpacing(this: Class) -> Float;

    /// Line spacing.
    fn Text_set_LineSpacing(this: Class, value: Float);

    /// Anchor.
    fn Text_get_Anchor(this: Class) -> Enumeration;

    /// Anchor.
    fn Text_set_Anchor(this: Class, value: Enumeration);

    /// Set the color
    fn Text_get_Color(this: Class) -> Tuple;

    /// Set the color
    fn Text_set_Color(this: Class, value: Tuple);

    /// Reference frame for the positions of the object.
    fn Text_get_ReferenceFrame(this: Class) -> Class;

    /// Reference frame for the positions of the object.
    fn Text_set_ReferenceFrame(this: Class, value: Class);

    /// Whether the object is visible.
    fn Text_get_Visible(this: Class) -> Bool;

    /// Whether the object is visible.
    fn Text_set_Visible(this: Class, value: Bool);

    /// Material used to render the object.
    /// Creates the material from a shader with the given name.
    fn Text_get_Material(this: Class) -> String;

    /// Material used to render the object.
    /// Creates the material from a shader with the given name.
    fn Text_set_Material(this: Class, value: String);

}

/// This service provides functionality to interact with
/// <a
/// href="https://forum.kerbalspaceprogram.com/index.php?/topic/104535-112-magic-smoke-industries-infernal-robotics-202/">Infernal Robotics</a>.
/// 
mod InfernalRobotics {
    /// Represents a servo. Obtained using
    /// InfernalRobotics.ServoGroup.Servos,
    /// InfernalRobotics.ServoGroup.ServoWithName
    /// or InfernalRobotics.ServoWithName.
    struct Servo;

    /// A group of servos, obtained by calling InfernalRobotics.ServoGroups
    /// or InfernalRobotics.ServoGroupWithName. Represents the "Servo Groups"
    /// in the InfernalRobotics UI.
    struct ServoGroup;

    /// A list of all the servo groups in the given <paramref name="vessel.
    fn ServoGroups(vessel: Class) -> List;

    /// Returns the servo group in the given <paramref name="vessel with the
    /// given <paramref name="name,
    /// or <c>null</c> if none exists. If multiple servo groups have the same
    /// name, only one of them is returned.
    /// 
    /// <param name="vessel">Vessel to check.</param>
    /// <param name="name">Name of servo group to find.</param>
    fn ServoGroupWithName(vessel: Class, name: String) -> Option<Class>;

    /// Returns the servo in the given <paramref name="vessel with the given
    /// <paramref name="name or
    /// <c>null</c> if none exists. If multiple servos have the same name, only
    /// one of them is returned.
    /// 
    /// <param name="vessel">Vessel to check.</param>
    /// <param name="name">Name of the servo to find.</param>
    fn ServoWithName(vessel: Class, name: String) -> Option<Class>;

    /// Whether Infernal Robotics is installed.
    fn get_Available() -> Bool;

    /// Whether Infernal Robotics API is ready.
    fn get_Ready() -> Bool;

    /// Moves the servo to the right.
    fn Servo_MoveRight(this: Class);

    /// Moves the servo to the left.
    fn Servo_MoveLeft(this: Class);

    /// Moves the servo to the center.
    fn Servo_MoveCenter(this: Class);

    /// Moves the servo to the next preset.
    fn Servo_MoveNextPreset(this: Class);

    /// Moves the servo to the previous preset.
    fn Servo_MovePrevPreset(this: Class);

    /// Moves the servo to <paramref name="position and sets the
    /// speed multiplier to <paramref name="speed.
    /// 
    /// <param name="position">The position to move the servo to.</param>
    /// <param name="speed">Speed multiplier for the movement.</param>
    fn Servo_MoveTo(this: Class, position: Float, speed: Float);

    /// Stops the servo.
    fn Servo_Stop(this: Class);

    /// The name of the servo.
    fn Servo_get_Name(this: Class) -> String;

    /// The name of the servo.
    fn Servo_set_Name(this: Class, value: String);

    /// The part containing the servo.
    fn Servo_get_Part(this: Class) -> Class;

    /// Whether the servo should be highlighted in-game.
    fn Servo_set_Highlight(this: Class, value: Bool);

    /// The position of the servo.
    fn Servo_get_Position(this: Class) -> Float;

    /// The minimum position of the servo, specified by the part configuration.
    fn Servo_get_MinConfigPosition(this: Class) -> Float;

    /// The maximum position of the servo, specified by the part configuration.
    fn Servo_get_MaxConfigPosition(this: Class) -> Float;

    /// The minimum position of the servo, specified by the in-game tweak menu.
    fn Servo_get_MinPosition(this: Class) -> Float;

    /// The minimum position of the servo, specified by the in-game tweak menu.
    fn Servo_set_MinPosition(this: Class, value: Float);

    /// The maximum position of the servo, specified by the in-game tweak menu.
    fn Servo_get_MaxPosition(this: Class) -> Float;

    /// The maximum position of the servo, specified by the in-game tweak menu.
    fn Servo_set_MaxPosition(this: Class, value: Float);

    /// The speed multiplier of the servo, specified by the part configuration.
    fn Servo_get_ConfigSpeed(this: Class) -> Float;

    /// The speed multiplier of the servo, specified by the in-game tweak menu.
    fn Servo_get_Speed(this: Class) -> Float;

    /// The speed multiplier of the servo, specified by the in-game tweak menu.
    fn Servo_set_Speed(this: Class, value: Float);

    /// The current speed at which the servo is moving.
    fn Servo_get_CurrentSpeed(this: Class) -> Float;

    /// The current speed at which the servo is moving.
    fn Servo_set_CurrentSpeed(this: Class, value: Float);

    /// The current speed multiplier set in the UI.
    fn Servo_get_Acceleration(this: Class) -> Float;

    /// The current speed multiplier set in the UI.
    fn Servo_set_Acceleration(this: Class, value: Float);

    /// Whether the servo is moving.
    fn Servo_get_IsMoving(this: Class) -> Bool;

    /// Whether the servo is freely moving.
    fn Servo_get_IsFreeMoving(this: Class) -> Bool;

    /// Whether the servo is locked.
    fn Servo_get_IsLocked(this: Class) -> Bool;

    /// Whether the servo is locked.
    fn Servo_set_IsLocked(this: Class, value: Bool);

    /// Whether the servos axis is inverted.
    fn Servo_get_IsAxisInverted(this: Class) -> Bool;

    /// Whether the servos axis is inverted.
    fn Servo_set_IsAxisInverted(this: Class, value: Bool);

    /// Returns the servo with the given <paramref name="name from this group,
    /// or <c>null</c> if none exists.
    /// 
    /// <param name="name">Name of servo to find.</param>
    fn ServoGroup_ServoWithName(this: Class, name: String) -> Option<Class>;

    /// Moves all of the servos in the group to the right.
    fn ServoGroup_MoveRight(this: Class);

    /// Moves all of the servos in the group to the left.
    fn ServoGroup_MoveLeft(this: Class);

    /// Moves all of the servos in the group to the center.
    fn ServoGroup_MoveCenter(this: Class);

    /// Moves all of the servos in the group to the next preset.
    fn ServoGroup_MoveNextPreset(this: Class);

    /// Moves all of the servos in the group to the previous preset.
    fn ServoGroup_MovePrevPreset(this: Class);

    /// Stops the servos in the group.
    fn ServoGroup_Stop(this: Class);

    /// The name of the group.
    fn ServoGroup_get_Name(this: Class) -> String;

    /// The name of the group.
    fn ServoGroup_set_Name(this: Class, value: String);

    /// The key assigned to be the "forward" key for the group.
    fn ServoGroup_get_ForwardKey(this: Class) -> String;

    /// The key assigned to be the "forward" key for the group.
    fn ServoGroup_set_ForwardKey(this: Class, value: String);

    /// The key assigned to be the "reverse" key for the group.
    fn ServoGroup_get_ReverseKey(this: Class) -> String;

    /// The key assigned to be the "reverse" key for the group.
    fn ServoGroup_set_ReverseKey(this: Class, value: String);

    /// The speed multiplier for the group.
    fn ServoGroup_get_Speed(this: Class) -> Float;

    /// The speed multiplier for the group.
    fn ServoGroup_set_Speed(this: Class, value: Float);

    /// Whether the group is expanded in the InfernalRobotics UI.
    fn ServoGroup_get_Expanded(this: Class) -> Bool;

    /// Whether the group is expanded in the InfernalRobotics UI.
    fn ServoGroup_set_Expanded(this: Class, value: Bool);

    /// The servos that are in the group.
    fn ServoGroup_get_Servos(this: Class) -> List;

    /// The parts containing the servos in the group.
    fn ServoGroup_get_Parts(this: Class) -> List;

}

/// This service provides functionality to interact with
/// <a
/// href="https://forum.kerbalspaceprogram.com/index.php?/topic/22809-13x-kerbal-alarm-clock-v3850-may-30/">Kerbal Alarm Clock</a>.
/// 
mod KerbalAlarmClock {
    /// Represents an alarm. Obtained by calling
    /// KerbalAlarmClock.Alarms,
    /// KerbalAlarmClock.AlarmWithName or
    /// KerbalAlarmClock.AlarmsWithType.
    struct Alarm;

    /// The action performed by an alarm when it fires.
    enum AlarmAction {
        /// Don't do anything at all...
        DoNothing = 0,
        /// Don't do anything, and delete the alarm.
        DoNothingDeleteWhenPassed = 1,
        /// Drop out of time warp.
        KillWarp = 2,
        /// Drop out of time warp.
        KillWarpOnly = 3,
        /// Display a message.
        MessageOnly = 4,
        /// Pause the game.
        PauseGame = 5,
    }

    /// The type of an alarm.
    enum AlarmType {
        /// An alarm for a specific date/time or a specific period in the
        /// future.
        Raw = 0,
        /// An alarm based on the next maneuver node on the current ships
        /// flight path.
        /// This node will be stored and can be restored when you come back to
        /// the ship.
        Maneuver = 1,
        /// See KerbalAlarmClock.AlarmType.Maneuver.
        ManeuverAuto = 2,
        /// An alarm for furthest part of the orbit from the planet.
        Apoapsis = 3,
        /// An alarm for nearest part of the orbit from the planet.
        Periapsis = 4,
        /// Ascending node for the targeted object, or equatorial ascending
        /// node.
        AscendingNode = 5,
        /// Descending node for the targeted object, or equatorial descending
        /// node.
        DescendingNode = 6,
        /// An alarm based on the closest approach of this vessel to the
        /// targeted
        /// vessel, some number of orbits into the future.
        Closest = 7,
        /// An alarm based on the expiry or deadline of contracts in career
        /// modes.
        Contract = 8,
        /// See KerbalAlarmClock.AlarmType.Contract.
        ContractAuto = 9,
        /// An alarm that is attached to a crew member.
        Crew = 10,
        /// An alarm that is triggered when a selected target comes within a
        /// chosen distance.
        Distance = 11,
        /// An alarm based on the time in the "Earth" alternative Universe (aka
        /// the Real World).
        EarthTime = 12,
        /// An alarm that fires as your landed craft passes under the orbit of
        /// your target.
        LaunchRendevous = 13,
        /// An alarm manually based on when the next SOI point is on the flight
        /// path
        /// or set to continually monitor the active flight path and add alarms
        /// as it
        /// detects SOI changes.
        SOIChange = 14,
        /// See KerbalAlarmClock.AlarmType.SOIChange.
        SOIChangeAuto = 15,
        /// An alarm based on Interplanetary Transfer Phase Angles, i.e. when
        /// should
        /// I launch to planet X? Based on Kosmo Not's post and used in Olex's
        /// Calculator.
        Transfer = 16,
        /// See KerbalAlarmClock.AlarmType.Transfer.
        TransferModelled = 17,
    }

    /// Get the alarm with the given <paramref name="name, or <c>null</c>
    /// if no alarms have that name. If more than one alarm has the name,
    /// only returns one of them.
    /// 
    /// <param name="name">Name of the alarm to search for.</param>
    fn AlarmWithName(name: String) -> Option<Class>;

    /// Get a list of alarms of the specified <paramref name="type.
    /// 
    /// <param name="type">Type of alarm to return.</param>
    fn AlarmsWithType(r#type: Enumeration) -> List;

    /// Create a new alarm and return it.
    /// 
    /// <param name="type">Type of the new alarm.</param>
    /// <param name="name">Name of the new alarm.</param>
    /// <param name="ut">Time at which the new alarm should trigger.</param>
    fn CreateAlarm(r#type: Enumeration, name: String, ut: Double) -> Class;

    /// Whether Kerbal Alarm Clock is available.
    fn get_Available() -> Bool;

    /// A list of all the alarms.
    fn get_Alarms() -> List;

    /// Removes the alarm.
    fn Alarm_Remove(this: Class);

    /// The action that the alarm triggers.
    fn Alarm_get_Action(this: Class) -> Enumeration;

    /// The action that the alarm triggers.
    fn Alarm_set_Action(this: Class, value: Enumeration);

    /// The number of seconds before the event that the alarm will fire.
    fn Alarm_get_Margin(this: Class) -> Double;

    /// The number of seconds before the event that the alarm will fire.
    fn Alarm_set_Margin(this: Class, value: Double);

    /// The time at which the alarm will fire.
    fn Alarm_get_Time(this: Class) -> Double;

    /// The time at which the alarm will fire.
    fn Alarm_set_Time(this: Class, value: Double);

    /// The type of the alarm.
    fn Alarm_get_Type(this: Class) -> Enumeration;

    /// The unique identifier for the alarm.
    fn Alarm_get_ID(this: Class) -> String;

    /// The short name of the alarm.
    fn Alarm_get_Name(this: Class) -> String;

    /// The short name of the alarm.
    fn Alarm_set_Name(this: Class, value: String);

    /// The long description of the alarm.
    fn Alarm_get_Notes(this: Class) -> String;

    /// The long description of the alarm.
    fn Alarm_set_Notes(this: Class, value: String);

    /// The number of seconds until the alarm will fire.
    fn Alarm_get_Remaining(this: Class) -> Double;

    /// Whether the alarm will be repeated after it has fired.
    fn Alarm_get_Repeat(this: Class) -> Bool;

    /// Whether the alarm will be repeated after it has fired.
    fn Alarm_set_Repeat(this: Class, value: Bool);

    /// The time delay to automatically create an alarm after it has fired.
    fn Alarm_get_RepeatPeriod(this: Class) -> Double;

    /// The time delay to automatically create an alarm after it has fired.
    fn Alarm_set_RepeatPeriod(this: Class, value: Double);

    /// The vessel that the alarm is attached to.
    fn Alarm_get_Vessel(this: Class) -> Class;

    /// The vessel that the alarm is attached to.
    fn Alarm_set_Vessel(this: Class, value: Class);

    /// The celestial body the vessel is departing from.
    fn Alarm_get_XferOriginBody(this: Class) -> Class;

    /// The celestial body the vessel is departing from.
    fn Alarm_set_XferOriginBody(this: Class, value: Class);

    /// The celestial body the vessel is arriving at.
    fn Alarm_get_XferTargetBody(this: Class) -> Class;

    /// The celestial body the vessel is arriving at.
    fn Alarm_set_XferTargetBody(this: Class, value: Class);

}

/// This service provides functionality to interact with
/// <a
/// href="https://forum.kerbalspaceprogram.com/index.php?/topic/139167-13-remotetech-v188-2017-09-03/">RemoteTech</a>.
/// 
mod RemoteTech {
    /// A RemoteTech antenna. Obtained by calling RemoteTech.Comms.Antennas or
    /// RemoteTech.Antenna.
    struct Antenna;

    /// Communications for a vessel.
    struct Comms;

    /// The type of object an antenna is targetting.
    /// See RemoteTech.Antenna.Target.
    enum Target {
        /// The active vessel.
        ActiveVessel = 0,
        /// A celestial body.
        CelestialBody = 1,
        /// A ground station.
        GroundStation = 2,
        /// A specific vessel.
        Vessel = 3,
        /// No target.
        None = 4,
    }

    /// Get a communications object, representing the communication capability
    /// of a particular vessel.
    fn Comms(vessel: Class) -> Class;

    /// Get the antenna object for a particular part.
    fn Antenna(part: Class) -> Class;

    /// Whether RemoteTech is installed.
    fn get_Available() -> Bool;

    /// The names of the ground stations.
    fn get_GroundStations() -> List;

    /// Get the part containing this antenna.
    fn Antenna_get_Part(this: Class) -> Class;

    /// Whether the antenna has a connection.
    fn Antenna_get_HasConnection(this: Class) -> Bool;

    /// The object that the antenna is targetting.
    /// This property can be used to set the target to RemoteTech.Target.None
    /// or RemoteTech.Target.ActiveVessel.
    /// To set the target to a celestial body, ground station or vessel see
    /// RemoteTech.Antenna.TargetBody,
    /// RemoteTech.Antenna.TargetGroundStation and
    /// RemoteTech.Antenna.TargetVessel.
    fn Antenna_get_Target(this: Class) -> Enumeration;

    /// The object that the antenna is targetting.
    /// This property can be used to set the target to RemoteTech.Target.None
    /// or RemoteTech.Target.ActiveVessel.
    /// To set the target to a celestial body, ground station or vessel see
    /// RemoteTech.Antenna.TargetBody,
    /// RemoteTech.Antenna.TargetGroundStation and
    /// RemoteTech.Antenna.TargetVessel.
    fn Antenna_set_Target(this: Class, value: Enumeration);

    /// The celestial body the antenna is targetting.
    fn Antenna_get_TargetBody(this: Class) -> Class;

    /// The celestial body the antenna is targetting.
    fn Antenna_set_TargetBody(this: Class, value: Class);

    /// The ground station the antenna is targetting.
    fn Antenna_get_TargetGroundStation(this: Class) -> String;

    /// The ground station the antenna is targetting.
    fn Antenna_set_TargetGroundStation(this: Class, value: String);

    /// The vessel the antenna is targetting.
    fn Antenna_get_TargetVessel(this: Class) -> Class;

    /// The vessel the antenna is targetting.
    fn Antenna_set_TargetVessel(this: Class, value: Class);

    /// The signal delay between the this vessel and another vessel, in seconds.
    /// 
    /// <param name="other"></param>
    fn Comms_SignalDelayToVessel(this: Class, other: Class) -> Double;

    /// Get the vessel.
    fn Comms_get_Vessel(this: Class) -> Class;

    /// Whether the vessel can be controlled locally.
    fn Comms_get_HasLocalControl(this: Class) -> Bool;

    /// Whether the vessel has a flight computer on board.
    fn Comms_get_HasFlightComputer(this: Class) -> Bool;

    /// Whether the vessel has any connection.
    fn Comms_get_HasConnection(this: Class) -> Bool;

    /// Whether the vessel has a connection to a ground station.
    fn Comms_get_HasConnectionToGroundStation(this: Class) -> Bool;

    /// The shortest signal delay to the vessel, in seconds.
    fn Comms_get_SignalDelay(this: Class) -> Double;

    /// The signal delay between the vessel and the closest ground station, in
    /// seconds.
    fn Comms_get_SignalDelayToGroundStation(this: Class) -> Double;

    /// The antennas for this vessel.
    fn Comms_get_Antennas(this: Class) -> List;

}

/// Provides functionality to interact with Kerbal Space Program. This includes
/// controlling
/// the active vessel, managing its resources, planning maneuver nodes and
/// auto-piloting.
mod SpaceCenter {
    /// Provides basic auto-piloting utilities for a vessel.
    /// Created by calling SpaceCenter.Vessel.AutoPilot.
    /// 
    /// <remarks>
    /// If a client engages the auto-pilot and then closes its connection to
    /// the server,
    /// the auto-pilot will be disengaged and its target reference frame,
    /// direction and roll
    /// reset to default.
    /// </remarks>
    struct AutoPilot;

    /// Controls the game's camera.
    /// Obtained by calling SpaceCenter.Camera.
    struct Camera;

    /// Represents a celestial body (such as a planet or moon).
    /// See SpaceCenter.Bodies.
    struct CelestialBody;

    /// Represents a communication node in the network. For example, a vessel
    /// or the KSC.
    struct CommLink;

    /// Represents a communication node in the network. For example, a vessel
    /// or the KSC.
    struct CommNode;

    /// Used to interact with CommNet for a given vessel.
    /// Obtained by calling SpaceCenter.Vessel.Comms.
    struct Comms;

    /// A contract. Can be accessed using SpaceCenter.ContractManager.
    struct Contract;

    /// Contracts manager.
    /// Obtained by calling SpaceCenter.ContractManager.
    struct ContractManager;

    /// A contract parameter. See SpaceCenter.Contract.Parameters.
    struct ContractParameter;

    /// Used to manipulate the controls of a vessel. This includes adjusting the
    /// throttle, enabling/disabling systems such as SAS and RCS, or altering
    /// the
    /// direction in which the vessel is pointing.
    /// Obtained by calling SpaceCenter.Vessel.Control.
    /// 
    /// <remarks>
    /// Control inputs (such as pitch, yaw and roll) are zeroed when all clients
    /// that have set one or more of these inputs are no longer connected.
    /// </remarks>
    struct Control;

    /// Represents crew in a vessel. Can be obtained using
    /// SpaceCenter.Vessel.Crew.
    struct CrewMember;

    /// Used to get flight telemetry for a vessel, by calling
    /// SpaceCenter.Vessel.Flight.
    /// All of the information returned by this class is given in the reference
    /// frame
    /// passed to that method.
    /// Obtained by calling SpaceCenter.Vessel.Flight.
    /// 
    /// <remarks>
    /// To get orbital information, such as the apoapsis or inclination, see
    /// <see cref="T:SpaceCenter.Orbit.
    /// </remarks>
    struct Flight;

    /// Represents a maneuver node. Can be created using
    /// SpaceCenter.Control.AddNode.
    struct Node;

    /// Describes an orbit. For example, the orbit of a vessel, obtained by
    /// calling
    /// SpaceCenter.Vessel.Orbit, or a celestial body, obtained by calling
    /// SpaceCenter.CelestialBody.Orbit.
    struct Orbit;

    /// An antenna. Obtained by calling SpaceCenter.Part.Antenna.
    struct Antenna;

    /// A cargo bay. Obtained by calling SpaceCenter.Part.CargoBay.
    struct CargoBay;

    /// An aerodynamic control surface. Obtained by calling
    /// SpaceCenter.Part.ControlSurface.
    struct ControlSurface;

    /// A decoupler. Obtained by calling SpaceCenter.Part.Decoupler
    struct Decoupler;

    /// A docking port. Obtained by calling SpaceCenter.Part.DockingPort
    struct DockingPort;

    /// An engine, including ones of various types.
    /// For example liquid fuelled gimballed engines, solid rocket boosters and
    /// jet engines.
    /// Obtained by calling SpaceCenter.Part.Engine.
    /// 
    /// <remarks>
    /// For RCS thrusters SpaceCenter.Part.RCS.
    /// </remarks>
    struct Engine;

    /// Obtained by calling SpaceCenter.Part.Experiment.
    struct Experiment;

    /// A fairing. Obtained by calling SpaceCenter.Part.Fairing.
    struct Fairing;

    /// Obtained by calling SpaceCenter.Part.AddForce.
    struct Force;

    /// An air intake. Obtained by calling SpaceCenter.Part.Intake.
    struct Intake;

    /// A launch clamp. Obtained by calling SpaceCenter.Part.LaunchClamp.
    struct LaunchClamp;

    /// A landing leg. Obtained by calling SpaceCenter.Part.Leg.
    struct Leg;

    /// A light. Obtained by calling SpaceCenter.Part.Light.
    struct Light;

    /// This can be used to interact with a specific part module. This includes
    /// part modules in
    /// stock KSP, and those added by mods.
    /// 
    /// In KSP, each part has zero or more
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/CFG_File_Documentation#MODULES">PartModules</a>
    /// 
    /// associated with it. Each one contains some of the functionality of the
    /// part.
    /// For example, an engine has a "ModuleEngines" part module that contains
    /// all the
    /// functionality of an engine.
    struct Module;

    /// A parachute. Obtained by calling SpaceCenter.Part.Parachute.
    struct Parachute;

    /// Represents an individual part. Vessels are made up of multiple parts.
    /// Instances of this class can be obtained by several methods in <see
    /// cref="T:SpaceCenter.Parts.
    struct Part;

    /// Instances of this class are used to interact with the parts of a vessel.
    /// An instance can be obtained by calling SpaceCenter.Vessel.Parts.
    struct Parts;

    /// A propellant for an engine. Obtains by calling
    /// SpaceCenter.Engine.Propellants.
    struct Propellant;

    /// An RCS block or thruster. Obtained by calling SpaceCenter.Part.RCS.
    struct RCS;

    /// A radiator. Obtained by calling SpaceCenter.Part.Radiator.
    struct Radiator;

    /// A reaction wheel. Obtained by calling SpaceCenter.Part.ReactionWheel.
    struct ReactionWheel;

    /// A resource converter. Obtained by calling
    /// SpaceCenter.Part.ResourceConverter.
    struct ResourceConverter;

    /// A resource harvester (drill). Obtained by calling
    /// SpaceCenter.Part.ResourceHarvester.
    struct ResourceHarvester;

    /// Obtained by calling SpaceCenter.Experiment.Data.
    struct ScienceData;

    /// Obtained by calling SpaceCenter.Experiment.ScienceSubject.
    struct ScienceSubject;

    /// A sensor, such as a thermometer. Obtained by calling
    /// SpaceCenter.Part.Sensor.
    struct Sensor;

    /// A solar panel. Obtained by calling SpaceCenter.Part.SolarPanel.
    struct SolarPanel;

    /// The component of an <see cref="T:SpaceCenter.Engine or <see
    /// cref="T:SpaceCenter.RCS part that generates thrust.
    /// Can obtained by calling SpaceCenter.Engine.Thrusters or
    /// SpaceCenter.RCS.Thrusters.
    /// 
    /// <remarks>
    /// Engines can consist of multiple thrusters.
    /// For example, the S3 KS-25x4 "Mammoth" has four rocket nozzels, and so
    /// consists of
    /// four thrusters.
    /// </remarks>
    struct Thruster;

    /// A wheel. Includes landing gear and rover wheels.
    /// Obtained by calling SpaceCenter.Part.Wheel.
    /// Can be used to control the motors, steering and deployment of wheels,
    /// among other things.
    struct Wheel;

    /// Represents a reference frame for positions, rotations and
    /// velocities. Contains:
    /// <list type="bullet"><item><description>The position of the
    /// origin.</description></item><item><description>The directions of the x,
    /// y and z axes.</description></item><item><description>The linear
    /// velocity of the frame.</description></item><item><description>The
    /// angular velocity of the frame.</description></item></list>
    /// <remarks>
    /// This class does not contain any properties or methods. It is only
    /// used as a parameter to other functions.
    /// </remarks>
    struct ReferenceFrame;

    /// An individual resource stored within a part.
    /// Created using methods in the <see cref="T:SpaceCenter.Resources class.
    struct Resource;

    /// Transfer resources between parts.
    struct ResourceTransfer;

    /// Represents the collection of resources stored in a vessel, stage or
    /// part.
    /// Created by calling SpaceCenter.Vessel.Resources,
    /// SpaceCenter.Vessel.ResourcesInDecoupleStage or
    /// SpaceCenter.Part.Resources.
    struct Resources;

    /// These objects are used to interact with vessels in KSP. This includes
    /// getting
    /// orbital and flight data, manipulating control inputs and managing
    /// resources.
    /// Created using SpaceCenter.ActiveVessel or SpaceCenter.Vessels.
    struct Vessel;

    /// Represents a waypoint. Can be created using
    /// SpaceCenter.WaypointManager.AddWaypoint.
    struct Waypoint;

    /// Waypoints are the location markers you can see on the map view showing
    /// you where contracts are targeted for.
    /// With this structure, you can obtain coordinate data for the locations
    /// of these waypoints.
    /// Obtained by calling SpaceCenter.WaypointManager.
    struct WaypointManager;

    /// See SpaceCenter.Camera.Mode.
    enum CameraMode {
        /// The camera is showing the active vessel, in "auto" mode.
        Automatic = 0,
        /// The camera is showing the active vessel, in "free" mode.
        Free = 1,
        /// The camera is showing the active vessel, in "chase" mode.
        Chase = 2,
        /// The camera is showing the active vessel, in "locked" mode.
        Locked = 3,
        /// The camera is showing the active vessel, in "orbital" mode.
        Orbital = 4,
        /// The Intra-Vehicular Activity view is being shown.
        IVA = 5,
        /// The map view is being shown.
        Map = 6,
    }

    /// The type of a communication link.
    /// See SpaceCenter.CommLink.Type.
    enum CommLinkType {
        /// Link is to a base station on Kerbin.
        Home = 0,
        /// Link is to a control source, for example a manned spacecraft.
        Control = 1,
        /// Link is to a relay satellite.
        Relay = 2,
    }

    /// The state of a contract. See SpaceCenter.Contract.State.
    enum ContractState {
        /// The contract is active.
        Active = 0,
        /// The contract has been canceled.
        Canceled = 1,
        /// The contract has been completed.
        Completed = 2,
        /// The deadline for the contract has expired.
        DeadlineExpired = 3,
        /// The contract has been declined.
        Declined = 4,
        /// The contract has been failed.
        Failed = 5,
        /// The contract has been generated.
        Generated = 6,
        /// The contract has been offered to the player.
        Offered = 7,
        /// The contract was offered to the player, but the offer expired.
        OfferExpired = 8,
        /// The contract has been withdrawn.
        Withdrawn = 9,
    }

    /// See SpaceCenter.Control.InputMode.
    enum ControlInputMode {
        /// Control inputs are added to the vessels current control inputs.
        Additive = 0,
        /// Control inputs (when they are non-zero) override the vessels
        /// current control inputs.
        Override = 1,
    }

    /// The control source of a vessel.
    /// See SpaceCenter.Control.Source.
    enum ControlSource {
        /// Vessel is controlled by a Kerbal.
        Kerbal = 0,
        /// Vessel is controlled by a probe core.
        Probe = 1,
        /// Vessel is not controlled.
        None = 2,
    }

    /// The control state of a vessel.
    /// See SpaceCenter.Control.State.
    enum ControlState {
        /// Full controllable.
        Full = 0,
        /// Partially controllable.
        Partial = 1,
        /// Not controllable.
        None = 2,
    }

    /// The type of a crew member.
    /// See SpaceCenter.CrewMember.Type.
    enum CrewMemberType {
        /// An applicant for crew.
        Applicant = 0,
        /// Rocket crew.
        Crew = 1,
        /// A tourist.
        Tourist = 2,
        /// An unowned crew member.
        Unowned = 3,
    }

    /// The game mode.
    /// Returned by <see cref="T:SpaceCenter.GameMode
    enum GameMode {
        /// Sandbox mode.
        Sandbox = 0,
        /// Career mode.
        Career = 1,
        /// Science career mode.
        Science = 2,
        /// Science sandbox mode.
        ScienceSandbox = 3,
        /// Mission mode.
        Mission = 4,
        /// Mission builder mode.
        MissionBuilder = 5,
        /// Scenario mode.
        Scenario = 6,
        /// Scenario mode that cannot be resumed.
        ScenarioNonResumable = 7,
    }

    /// The state of an antenna. See SpaceCenter.Antenna.State.
    enum AntennaState {
        /// Antenna is fully deployed.
        Deployed = 0,
        /// Antenna is fully retracted.
        Retracted = 1,
        /// Antenna is being deployed.
        Deploying = 2,
        /// Antenna is being retracted.
        Retracting = 3,
        /// Antenna is broken.
        Broken = 4,
    }

    /// The state of a cargo bay. See SpaceCenter.CargoBay.State.
    enum CargoBayState {
        /// Cargo bay is fully open.
        Open = 0,
        /// Cargo bay closed and locked.
        Closed = 1,
        /// Cargo bay is opening.
        Opening = 2,
        /// Cargo bay is closing.
        Closing = 3,
    }

    /// The state of a docking port. See SpaceCenter.DockingPort.State.
    enum DockingPortState {
        /// The docking port is ready to dock to another docking port.
        Ready = 0,
        /// The docking port is docked to another docking port, or docked to
        /// another part (from the VAB/SPH).
        Docked = 1,
        /// The docking port is very close to another docking port,
        /// but has not docked. It is using magnetic force to acquire a solid
        /// dock.
        Docking = 2,
        /// The docking port has just been undocked from another docking port,
        /// and is disabled until it moves away by a sufficient distance
        /// (SpaceCenter.DockingPort.ReengageDistance).
        Undocking = 3,
        /// The docking port has a shield, and the shield is closed.
        Shielded = 4,
        /// The docking ports shield is currently opening/closing.
        Moving = 5,
    }

    /// The state of a landing leg. See SpaceCenter.Leg.State.
    enum LegState {
        /// Landing leg is fully deployed.
        Deployed = 0,
        /// Landing leg is fully retracted.
        Retracted = 1,
        /// Landing leg is being deployed.
        Deploying = 2,
        /// Landing leg is being retracted.
        Retracting = 3,
        /// Landing leg is broken.
        Broken = 4,
    }

    /// The state of the motor on a powered wheel. See
    /// SpaceCenter.Wheel.MotorState.
    enum MotorState {
        /// The motor is idle.
        Idle = 0,
        /// The motor is running.
        Running = 1,
        /// The motor is disabled.
        Disabled = 2,
        /// The motor is inoperable.
        Inoperable = 3,
        /// The motor does not have enough resources to run.
        NotEnoughResources = 4,
    }

    /// The state of a parachute. See SpaceCenter.Parachute.State.
    enum ParachuteState {
        /// The parachute is safely tucked away inside its housing.
        Stowed = 0,
        /// The parachute is armed for deployment. (RealChutes only)
        Armed = 1,
        /// The parachute is still stowed, but ready to semi-deploy.
        /// (Stock parachutes only)
        Active = 2,
        /// The parachute has been deployed and is providing some drag,
        /// but is not fully deployed yet. (Stock parachutes only)
        SemiDeployed = 3,
        /// The parachute is fully deployed.
        Deployed = 4,
        /// The parachute has been cut.
        Cut = 5,
    }

    /// The state of a radiator. <see cref="T:SpaceCenter.RadiatorState
    enum RadiatorState {
        /// Radiator is fully extended.
        Extended = 0,
        /// Radiator is fully retracted.
        Retracted = 1,
        /// Radiator is being extended.
        Extending = 2,
        /// Radiator is being retracted.
        Retracting = 3,
        /// Radiator is being broken.
        Broken = 4,
    }

    /// The state of a resource converter. See
    /// SpaceCenter.ResourceConverter.State.
    enum ResourceConverterState {
        /// Converter is running.
        Running = 0,
        /// Converter is idle.
        Idle = 1,
        /// Converter is missing a required resource.
        MissingResource = 2,
        /// No available storage for output resource.
        StorageFull = 3,
        /// At preset resource capacity.
        Capacity = 4,
        /// Unknown state. Possible with modified resource converters.
        /// In this case, check SpaceCenter.ResourceConverter.StatusInfo for
        /// more information.
        Unknown = 5,
    }

    /// The state of a resource harvester. See
    /// SpaceCenter.ResourceHarvester.State.
    enum ResourceHarvesterState {
        /// The drill is deploying.
        Deploying = 0,
        /// The drill is deployed and ready.
        Deployed = 1,
        /// The drill is retracting.
        Retracting = 2,
        /// The drill is retracted.
        Retracted = 3,
        /// The drill is running.
        Active = 4,
    }

    /// The state of a solar panel. See SpaceCenter.SolarPanel.State.
    enum SolarPanelState {
        /// Solar panel is fully extended.
        Extended = 0,
        /// Solar panel is fully retracted.
        Retracted = 1,
        /// Solar panel is being extended.
        Extending = 2,
        /// Solar panel is being retracted.
        Retracting = 3,
        /// Solar panel is broken.
        Broken = 4,
    }

    /// The state of a wheel. See SpaceCenter.Wheel.State.
    enum WheelState {
        /// Wheel is fully deployed.
        Deployed = 0,
        /// Wheel is fully retracted.
        Retracted = 1,
        /// Wheel is being deployed.
        Deploying = 2,
        /// Wheel is being retracted.
        Retracting = 3,
        /// Wheel is broken.
        Broken = 4,
    }

    /// The way in which a resource flows between parts. See
    /// SpaceCenter.Resources.FlowMode.
    enum ResourceFlowMode {
        /// The resource flows to any part in the vessel. For example, electric
        /// charge.
        Vessel = 0,
        /// The resource flows from parts in the first stage, followed by the
        /// second,
        /// and so on. For example, mono-propellant.
        Stage = 1,
        /// The resource flows between adjacent parts within the vessel. For
        /// example,
        /// liquid fuel or oxidizer.
        Adjacent = 2,
        /// The resource does not flow. For example, solid fuel.
        None = 3,
    }

    /// The behavior of the SAS auto-pilot. See SpaceCenter.AutoPilot.SASMode.
    enum SASMode {
        /// Stability assist mode. Dampen out any rotation.
        StabilityAssist = 0,
        /// Point in the burn direction of the next maneuver node.
        Maneuver = 1,
        /// Point in the prograde direction.
        Prograde = 2,
        /// Point in the retrograde direction.
        Retrograde = 3,
        /// Point in the orbit normal direction.
        Normal = 4,
        /// Point in the orbit anti-normal direction.
        AntiNormal = 5,
        /// Point in the orbit radial direction.
        Radial = 6,
        /// Point in the orbit anti-radial direction.
        AntiRadial = 7,
        /// Point in the direction of the current target.
        Target = 8,
        /// Point away from the current target.
        AntiTarget = 9,
    }

    /// The mode of the speed reported in the navball.
    /// See SpaceCenter.Control.SpeedMode.
    enum SpeedMode {
        /// Speed is relative to the vessel's orbit.
        Orbit = 0,
        /// Speed is relative to the surface of the body being orbited.
        Surface = 1,
        /// Speed is relative to the current target.
        Target = 2,
    }

    /// The situation a vessel is in.
    /// See SpaceCenter.Vessel.Situation.
    enum VesselSituation {
        /// Vessel is awaiting launch.
        PreLaunch = 0,
        /// Vessel is orbiting a body.
        Orbiting = 1,
        /// Vessel is on a sub-orbital trajectory.
        SubOrbital = 2,
        /// Escaping.
        Escaping = 3,
        /// Vessel is flying through an atmosphere.
        Flying = 4,
        /// Vessel is landed on the surface of a body.
        Landed = 5,
        /// Vessel has splashed down in an ocean.
        Splashed = 6,
        /// Vessel is docked to another.
        Docked = 7,
    }

    /// The type of a vessel.
    /// See SpaceCenter.Vessel.Type.
    enum VesselType {
        /// Base.
        Base = 0,
        /// Debris.
        Debris = 1,
        /// Lander.
        Lander = 2,
        /// Plane.
        Plane = 3,
        /// Probe.
        Probe = 4,
        /// Relay.
        Relay = 5,
        /// Rover.
        Rover = 6,
        /// Ship.
        Ship = 7,
        /// Station.
        Station = 8,
    }

    /// The time warp mode.
    /// Returned by <see cref="T:SpaceCenter.WarpMode
    enum WarpMode {
        /// Time warp is active, and in regular "on-rails" mode.
        Rails = 0,
        /// Time warp is active, and in physical time warp mode.
        Physics = 1,
        /// Time warp is not active.
        None = 2,
    }

    /// Clears the current target.
    fn ClearTarget();

    /// Returns a list of vessels from the given <paramref name="craftDirectory
    /// that can be launched.
    /// 
    /// <param name="craftDirectory">Name of the directory in the current saves
    /// "Ships" directory. For example <c>"VAB"</c> or <c>"SPH"</c>.</param>
    fn LaunchableVessels(craftDirectory: String) -> List;

    /// Launch a vessel.
    /// 
    /// <param name="craftDirectory">Name of the directory in the current saves
    /// "Ships" directory, that contains the craft file.
    /// For example <c>"VAB"</c> or <c>"SPH"</c>.</param>
    /// <param name="name">Name of the vessel to launch. This is the name of
    /// the ".craft" file
    /// in the save directory, without the ".craft" file extension.</param>
    /// <param name="launchSite">Name of the launch site. For example
    /// <c>"LaunchPad"</c> or
    /// <c>"Runway"</c>.</param>
    /// <param name="recover">If true and there is a vessel on the launch site,
    /// recover it before launching.</param>
    /// <remarks>
    /// Throws an exception if any of the games pre-flight checks fail.
    /// </remarks>
    fn LaunchVessel(craftDirectory: String, name: String, launchSite: String, recover: Bool);

    /// Launch a new vessel from the VAB onto the launchpad.
    /// 
    /// <param name="name">Name of the vessel to launch.</param>
    /// <param name="recover">If true and there is a vessel on the launch pad,
    /// recover it before launching.</param>
    /// <remarks>
    /// This is equivalent to calling SpaceCenter.LaunchVessel with the craft
    /// directory
    /// set to "VAB" and the launch site set to "LaunchPad".
    /// Throws an exception if any of the games pre-flight checks fail.
    /// </remarks>
    fn LaunchVesselFromVAB(name: String, recover: Bool);

    /// Launch a new vessel from the SPH onto the runway.
    /// 
    /// <param name="name">Name of the vessel to launch.</param>
    /// <param name="recover">If true and there is a vessel on the runway,
    /// recover it before launching.</param>
    /// <remarks>
    /// This is equivalent to calling SpaceCenter.LaunchVessel with the craft
    /// directory
    /// set to "SPH" and the launch site set to "Runway".
    /// Throws an exception if any of the games pre-flight checks fail.
    /// </remarks>
    fn LaunchVesselFromSPH(name: String, recover: Bool);

    /// Save the game with a given name.
    /// This will create a save file called <c>name.sfs</c> in the folder of the
    /// current save game.
    fn Save(name: String);

    /// Load the game with the given name.
    /// This will create a load a save file called <c>name.sfs</c> from the
    /// folder of the
    /// current save game.
    fn Load(name: String);

    /// Save a quicksave.
    /// 
    /// <remarks>
    /// This is the same as calling SpaceCenter.Save with the name "quicksave".
    /// </remarks>
    fn Quicksave();

    /// Load a quicksave.
    /// 
    /// <remarks>
    /// This is the same as calling SpaceCenter.Load with the name "quicksave".
    /// </remarks>
    fn Quickload();

    /// Returns <c>true</c> if regular "on-rails" time warp can be used, at the
    /// specified warp
    /// <paramref name="factor. The maximum time warp rate is limited by
    /// various things,
    /// including how close the active vessel is to a planet. See
    /// <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">the KSP
    /// wiki</a>
    /// for details.
    /// 
    /// <param name="factor">The warp factor to check.</param>
    fn CanRailsWarpAt(factor: Sint32) -> Bool;

    /// Uses time acceleration to warp forward to a time in the future,
    /// specified
    /// by universal time <paramref name="ut. This call blocks until the desired
    /// time is reached. Uses regular "on-rails" or physical time warp as
    /// appropriate.
    /// For example, physical time warp is used when the active vessel is
    /// traveling
    /// through an atmosphere. When using regular "on-rails" time warp, the warp
    /// rate is limited by <paramref name="maxRailsRate, and when using physical
    /// time warp, the warp rate is limited by <paramref name="maxPhysicsRate.
    /// 
    /// <param name="ut">The universal time to warp to, in seconds.</param>
    /// <param name="maxRailsRate">The maximum warp rate in regular "on-rails"
    /// time warp.
    /// </param>
    /// <param name="maxPhysicsRate">The maximum warp rate in physical time
    /// warp.</param>
    /// # Returns
    /// 
    /// When the time warp is complete.
    fn WarpTo(ut: Double, maxRailsRate: Float, maxPhysicsRate: Float);

    /// Converts a position from one reference frame to another.
    /// 
    /// <param name="position">Position, as a vector, in reference frame
    /// <paramref name="from.</param>
    /// <param name="from">The reference frame that the position is in.</param>
    /// <param name="to">The reference frame to covert the position to.</param>
    /// # Returns
    /// 
    /// The corresponding position, as a vector, in reference frame
    /// <paramref name="to.
    fn TransformPosition(position: Tuple, from: Class, to: Class) -> Tuple;

    /// Converts a direction from one reference frame to another.
    /// 
    /// <param name="direction">Direction, as a vector, in reference frame
    /// <paramref name="from. </param>
    /// <param name="from">The reference frame that the direction is in.</param>
    /// <param name="to">The reference frame to covert the direction to.</param>
    /// # Returns
    /// 
    /// The corresponding direction, as a vector, in reference frame
    /// <paramref name="to.
    fn TransformDirection(direction: Tuple, from: Class, to: Class) -> Tuple;

    /// Converts a rotation from one reference frame to another.
    /// 
    /// <param name="rotation">Rotation, as a quaternion of the form <math>(x,
    /// y, z, w)</math>,
    /// in reference frame <paramref name="from.</param>
    /// <param name="from">The reference frame that the rotation is in.</param>
    /// <param name="to">The reference frame to covert the rotation to.</param>
    /// # Returns
    /// 
    /// The corresponding rotation, as a quaternion of the form
    /// <math>(x, y, z, w)</math>, in reference frame <paramref name="to.
    fn TransformRotation(rotation: Tuple, from: Class, to: Class) -> Tuple;

    /// Converts a velocity (acting at the specified position) from one
    /// reference frame
    /// to another. The position is required to take the relative angular
    /// velocity of the
    /// reference frames into account.
    /// 
    /// <param name="position">Position, as a vector, in reference frame
    /// <paramref name="from.</param>
    /// <param name="velocity">Velocity, as a vector that points in the
    /// direction of travel and
    /// whose magnitude is the speed in meters per second, in reference frame
    /// <paramref name="from.</param>
    /// <param name="from">The reference frame that the position and velocity
    /// are in.</param>
    /// <param name="to">The reference frame to covert the velocity to.</param>
    /// # Returns
    /// 
    /// The corresponding velocity, as a vector, in reference frame
    /// <paramref name="to.
    fn TransformVelocity(position: Tuple, velocity: Tuple, from: Class, to: Class) -> Tuple;

    /// Cast a ray from a given position in a given direction, and return the
    /// distance to the hit point.
    /// If no hit occurs, returns infinity.
    /// 
    /// <param name="position">Position, as a vector, of the origin of the
    /// ray.</param>
    /// <param name="direction">Direction of the ray, as a unit vector.</param>
    /// <param name="referenceFrame">The reference frame that the position and
    /// direction are in.</param>
    /// # Returns
    /// 
    /// The distance to the hit, in meters, or infinity if there was no hit.
    fn RaycastDistance(position: Tuple, direction: Tuple, referenceFrame: Class) -> Double;

    /// Cast a ray from a given position in a given direction, and return the
    /// part that it hits.
    /// If no hit occurs, returns <c>null</c>.
    /// 
    /// <param name="position">Position, as a vector, of the origin of the
    /// ray.</param>
    /// <param name="direction">Direction of the ray, as a unit vector.</param>
    /// <param name="referenceFrame">The reference frame that the position and
    /// direction are in.</param>
    /// # Returns
    /// 
    /// The part that was hit or <c>null</c> if there was no hit.
    fn RaycastPart(position: Tuple, direction: Tuple, referenceFrame: Class) -> Option<Class>;

    /// The current mode the game is in.
    fn get_GameMode() -> Enumeration;

    /// The current amount of science.
    fn get_Science() -> Float;

    /// The current amount of funds.
    fn get_Funds() -> Double;

    /// The current amount of reputation.
    fn get_Reputation() -> Float;

    /// The currently active vessel.
    fn get_ActiveVessel() -> Class;

    /// The currently active vessel.
    fn set_ActiveVessel(value: Class);

    /// A list of all the vessels in the game.
    fn get_Vessels() -> List;

    /// A dictionary of all celestial bodies (planets, moons, etc.) in the game,
    /// keyed by the name of the body.
    fn get_Bodies() -> Dictionary;

    /// The currently targeted celestial body.
    fn get_TargetBody() -> Option<Class>;

    /// The currently targeted celestial body.
    fn set_TargetBody(value: Class);

    /// The currently targeted vessel.
    fn get_TargetVessel() -> Option<Class>;

    /// The currently targeted vessel.
    fn set_TargetVessel(value: Class);

    /// The currently targeted docking port.
    fn get_TargetDockingPort() -> Option<Class>;

    /// The currently targeted docking port.
    fn set_TargetDockingPort(value: Class);

    /// The waypoint manager.
    fn get_WaypointManager() -> Class;

    /// The contract manager.
    fn get_ContractManager() -> Class;

    /// An object that can be used to control the camera.
    fn get_Camera() -> Class;

    /// Whether the UI is visible.
    fn get_UIVisible() -> Bool;

    /// Whether the UI is visible.
    fn set_UIVisible(value: Bool);

    /// Whether the navball is visible.
    fn get_Navball() -> Bool;

    /// Whether the navball is visible.
    fn set_Navball(value: Bool);

    /// The current universal time in seconds.
    fn get_UT() -> Double;

    /// The value of the <a
    /// href="https://en.wikipedia.org/wiki/Gravitational_constant">
    /// gravitational constant</a> G in <math>N(m/kg)^2</math>.
    fn get_G() -> Double;

    /// The current time warp mode. Returns SpaceCenter.WarpMode.None if time
    /// warp is not active, SpaceCenter.WarpMode.Rails if regular "on-rails"
    /// time warp
    /// is active, or SpaceCenter.WarpMode.Physics if physical time warp is
    /// active.
    fn get_WarpMode() -> Enumeration;

    /// The current warp rate. This is the rate at which time is passing for
    /// either on-rails or physical time warp. For example, a value of 10 means
    /// time is passing 10x faster than normal. Returns 1 if time warp is not
    /// active.
    fn get_WarpRate() -> Float;

    /// The current warp factor. This is the index of the rate at which time
    /// is passing for either regular "on-rails" or physical time warp. Returns
    /// 0
    /// if time warp is not active. When in on-rails time warp, this is equal to
    /// SpaceCenter.RailsWarpFactor, and in physics time warp, this is equal to
    /// SpaceCenter.PhysicsWarpFactor.
    fn get_WarpFactor() -> Float;

    /// The time warp rate, using regular "on-rails" time warp. A value between
    /// 0 and 7 inclusive. 0 means no time warp. Returns 0 if physical time warp
    /// is active.
    /// 
    /// If requested time warp factor cannot be set, it will be set to the next
    /// lowest possible value. For example, if the vessel is too close to a
    /// planet. See <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">
    /// the KSP wiki</a> for details.
    fn get_RailsWarpFactor() -> Sint32;

    /// The time warp rate, using regular "on-rails" time warp. A value between
    /// 0 and 7 inclusive. 0 means no time warp. Returns 0 if physical time warp
    /// is active.
    /// 
    /// If requested time warp factor cannot be set, it will be set to the next
    /// lowest possible value. For example, if the vessel is too close to a
    /// planet. See <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">
    /// the KSP wiki</a> for details.
    fn set_RailsWarpFactor(value: Sint32);

    /// The physical time warp rate. A value between 0 and 3 inclusive. 0 means
    /// no time warp. Returns 0 if regular "on-rails" time warp is active.
    fn get_PhysicsWarpFactor() -> Sint32;

    /// The physical time warp rate. A value between 0 and 3 inclusive. 0 means
    /// no time warp. Returns 0 if regular "on-rails" time warp is active.
    fn set_PhysicsWarpFactor(value: Sint32);

    /// The current maximum regular "on-rails" warp factor that can be set.
    /// A value between 0 and 7 inclusive. See
    /// <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">the KSP
    /// wiki</a>
    /// for details.
    fn get_MaximumRailsWarpFactor() -> Sint32;

    /// Whether <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a> is installed.
    /// 
    fn get_FARAvailable() -> Bool;

    /// Engage the auto-pilot.
    fn AutoPilot_Engage(this: Class);

    /// Disengage the auto-pilot.
    fn AutoPilot_Disengage(this: Class);

    /// Blocks until the vessel is pointing in the target direction and has
    /// the target roll (if set). Throws an exception if the auto-pilot has not
    /// been engaged.
    fn AutoPilot_Wait(this: Class);

    /// Set target pitch and heading angles.
    /// 
    /// <param name="pitch">Target pitch angle, in degrees between -90° and
    /// +90°.</param>
    /// <param name="heading">Target heading angle, in degrees between 0° and
    /// 360°.</param>
    fn AutoPilot_TargetPitchAndHeading(this: Class, pitch: Float, heading: Float);

    /// The error, in degrees, between the direction the ship has been asked
    /// to point in and the direction it is pointing in. Throws an exception if
    /// the auto-pilot
    /// has not been engaged and SAS is not enabled or is in stability assist
    /// mode.
    fn AutoPilot_get_Error(this: Class) -> Float;

    /// The error, in degrees, between the vessels current and target pitch.
    /// Throws an exception if the auto-pilot has not been engaged.
    fn AutoPilot_get_PitchError(this: Class) -> Float;

    /// The error, in degrees, between the vessels current and target heading.
    /// Throws an exception if the auto-pilot has not been engaged.
    fn AutoPilot_get_HeadingError(this: Class) -> Float;

    /// The error, in degrees, between the vessels current and target roll.
    /// Throws an exception if the auto-pilot has not been engaged or no target
    /// roll is set.
    fn AutoPilot_get_RollError(this: Class) -> Float;

    /// The reference frame for the target direction
    /// (SpaceCenter.AutoPilot.TargetDirection).
    /// 
    /// <remarks>
    /// An error will be thrown if this property is set to a reference frame
    /// that rotates with
    /// the vessel being controlled, as it is impossible to rotate the vessel
    /// in such a
    /// reference frame.
    /// </remarks>
    fn AutoPilot_get_ReferenceFrame(this: Class) -> Class;

    /// The reference frame for the target direction
    /// (SpaceCenter.AutoPilot.TargetDirection).
    /// 
    /// <remarks>
    /// An error will be thrown if this property is set to a reference frame
    /// that rotates with
    /// the vessel being controlled, as it is impossible to rotate the vessel
    /// in such a
    /// reference frame.
    /// </remarks>
    fn AutoPilot_set_ReferenceFrame(this: Class, value: Class);

    /// The target pitch, in degrees, between -90° and +90°.
    fn AutoPilot_get_TargetPitch(this: Class) -> Float;

    /// The target pitch, in degrees, between -90° and +90°.
    fn AutoPilot_set_TargetPitch(this: Class, value: Float);

    /// The target heading, in degrees, between 0° and 360°.
    fn AutoPilot_get_TargetHeading(this: Class) -> Float;

    /// The target heading, in degrees, between 0° and 360°.
    fn AutoPilot_set_TargetHeading(this: Class, value: Float);

    /// The target roll, in degrees. <c>NaN</c> if no target roll is set.
    fn AutoPilot_get_TargetRoll(this: Class) -> Float;

    /// The target roll, in degrees. <c>NaN</c> if no target roll is set.
    fn AutoPilot_set_TargetRoll(this: Class, value: Float);

    /// Direction vector corresponding to the target pitch and heading.
    /// This is in the reference frame specified by <see
    /// cref="T:SpaceCenter.ReferenceFrame.
    fn AutoPilot_get_TargetDirection(this: Class) -> Tuple;

    /// Direction vector corresponding to the target pitch and heading.
    /// This is in the reference frame specified by <see
    /// cref="T:SpaceCenter.ReferenceFrame.
    fn AutoPilot_set_TargetDirection(this: Class, value: Tuple);

    /// The state of SAS.
    /// 
    /// <remarks>Equivalent to SpaceCenter.Control.SAS</remarks>
    fn AutoPilot_get_SAS(this: Class) -> Bool;

    /// The state of SAS.
    /// 
    /// <remarks>Equivalent to SpaceCenter.Control.SAS</remarks>
    fn AutoPilot_set_SAS(this: Class, value: Bool);

    /// The current <see cref="T:SpaceCenter.SASMode.
    /// These modes are equivalent to the mode buttons to the left of the
    /// navball that appear
    /// when SAS is enabled.
    /// 
    /// <remarks>Equivalent to SpaceCenter.Control.SASMode</remarks>
    fn AutoPilot_get_SASMode(this: Class) -> Enumeration;

    /// The current <see cref="T:SpaceCenter.SASMode.
    /// These modes are equivalent to the mode buttons to the left of the
    /// navball that appear
    /// when SAS is enabled.
    /// 
    /// <remarks>Equivalent to SpaceCenter.Control.SASMode</remarks>
    fn AutoPilot_set_SASMode(this: Class, value: Enumeration);

    /// The threshold at which the autopilot will try to match the target roll
    /// angle, if any.
    /// Defaults to 5 degrees.
    fn AutoPilot_get_RollThreshold(this: Class) -> Double;

    /// The threshold at which the autopilot will try to match the target roll
    /// angle, if any.
    /// Defaults to 5 degrees.
    fn AutoPilot_set_RollThreshold(this: Class, value: Double);

    /// The maximum amount of time that the vessel should need to come to a
    /// complete stop.
    /// This determines the maximum angular velocity of the vessel.
    /// A vector of three stopping times, in seconds, one for each of the
    /// pitch, roll
    /// and yaw axes. Defaults to 0.5 seconds for each axis.
    fn AutoPilot_get_StoppingTime(this: Class) -> Tuple;

    /// The maximum amount of time that the vessel should need to come to a
    /// complete stop.
    /// This determines the maximum angular velocity of the vessel.
    /// A vector of three stopping times, in seconds, one for each of the
    /// pitch, roll
    /// and yaw axes. Defaults to 0.5 seconds for each axis.
    fn AutoPilot_set_StoppingTime(this: Class, value: Tuple);

    /// The time the vessel should take to come to a stop pointing in the
    /// target direction.
    /// This determines the angular acceleration used to decelerate the vessel.
    /// A vector of three times, in seconds, one for each of the pitch, roll
    /// and yaw axes.
    /// Defaults to 5 seconds for each axis.
    fn AutoPilot_get_DecelerationTime(this: Class) -> Tuple;

    /// The time the vessel should take to come to a stop pointing in the
    /// target direction.
    /// This determines the angular acceleration used to decelerate the vessel.
    /// A vector of three times, in seconds, one for each of the pitch, roll
    /// and yaw axes.
    /// Defaults to 5 seconds for each axis.
    fn AutoPilot_set_DecelerationTime(this: Class, value: Tuple);

    /// The angle at which the autopilot considers the vessel to be pointing
    /// close to the target.
    /// This determines the midpoint of the target velocity attenuation
    /// function.
    /// A vector of three angles, in degrees, one for each of the pitch, roll
    /// and yaw axes.
    /// Defaults to 1° for each axis.
    fn AutoPilot_get_AttenuationAngle(this: Class) -> Tuple;

    /// The angle at which the autopilot considers the vessel to be pointing
    /// close to the target.
    /// This determines the midpoint of the target velocity attenuation
    /// function.
    /// A vector of three angles, in degrees, one for each of the pitch, roll
    /// and yaw axes.
    /// Defaults to 1° for each axis.
    fn AutoPilot_set_AttenuationAngle(this: Class, value: Tuple);

    /// Whether the rotation rate controllers PID parameters should be
    /// automatically tuned
    /// using the vessels moment of inertia and available torque. Defaults to
    /// <c>true</c>.
    /// See SpaceCenter.AutoPilot.TimeToPeak and
    /// SpaceCenter.AutoPilot.Overshoot.
    fn AutoPilot_get_AutoTune(this: Class) -> Bool;

    /// Whether the rotation rate controllers PID parameters should be
    /// automatically tuned
    /// using the vessels moment of inertia and available torque. Defaults to
    /// <c>true</c>.
    /// See SpaceCenter.AutoPilot.TimeToPeak and
    /// SpaceCenter.AutoPilot.Overshoot.
    fn AutoPilot_set_AutoTune(this: Class, value: Bool);

    /// The target time to peak used to autotune the PID controllers.
    /// A vector of three times, in seconds, for each of the pitch, roll and
    /// yaw axes.
    /// Defaults to 3 seconds for each axis.
    fn AutoPilot_get_TimeToPeak(this: Class) -> Tuple;

    /// The target time to peak used to autotune the PID controllers.
    /// A vector of three times, in seconds, for each of the pitch, roll and
    /// yaw axes.
    /// Defaults to 3 seconds for each axis.
    fn AutoPilot_set_TimeToPeak(this: Class, value: Tuple);

    /// The target overshoot percentage used to autotune the PID controllers.
    /// A vector of three values, between 0 and 1, for each of the pitch, roll
    /// and yaw axes.
    /// Defaults to 0.01 for each axis.
    fn AutoPilot_get_Overshoot(this: Class) -> Tuple;

    /// The target overshoot percentage used to autotune the PID controllers.
    /// A vector of three values, between 0 and 1, for each of the pitch, roll
    /// and yaw axes.
    /// Defaults to 0.01 for each axis.
    fn AutoPilot_set_Overshoot(this: Class, value: Tuple);

    /// Gains for the pitch PID controller.
    /// 
    /// <remarks>
    /// When SpaceCenter.AutoPilot.AutoTune is true, these values are updated
    /// automatically,
    /// which will overwrite any manual changes.
    /// </remarks>
    fn AutoPilot_get_PitchPIDGains(this: Class) -> Tuple;

    /// Gains for the pitch PID controller.
    /// 
    /// <remarks>
    /// When SpaceCenter.AutoPilot.AutoTune is true, these values are updated
    /// automatically,
    /// which will overwrite any manual changes.
    /// </remarks>
    fn AutoPilot_set_PitchPIDGains(this: Class, value: Tuple);

    /// Gains for the roll PID controller.
    /// 
    /// <remarks>
    /// When SpaceCenter.AutoPilot.AutoTune is true, these values are updated
    /// automatically,
    /// which will overwrite any manual changes.
    /// </remarks>
    fn AutoPilot_get_RollPIDGains(this: Class) -> Tuple;

    /// Gains for the roll PID controller.
    /// 
    /// <remarks>
    /// When SpaceCenter.AutoPilot.AutoTune is true, these values are updated
    /// automatically,
    /// which will overwrite any manual changes.
    /// </remarks>
    fn AutoPilot_set_RollPIDGains(this: Class, value: Tuple);

    /// Gains for the yaw PID controller.
    /// 
    /// <remarks>
    /// When SpaceCenter.AutoPilot.AutoTune is true, these values are updated
    /// automatically,
    /// which will overwrite any manual changes.
    /// </remarks>
    fn AutoPilot_get_YawPIDGains(this: Class) -> Tuple;

    /// Gains for the yaw PID controller.
    /// 
    /// <remarks>
    /// When SpaceCenter.AutoPilot.AutoTune is true, these values are updated
    /// automatically,
    /// which will overwrite any manual changes.
    /// </remarks>
    fn AutoPilot_set_YawPIDGains(this: Class, value: Tuple);

    /// The current mode of the camera.
    fn Camera_get_Mode(this: Class) -> Enumeration;

    /// The current mode of the camera.
    fn Camera_set_Mode(this: Class, value: Enumeration);

    /// The pitch of the camera, in degrees.
    /// A value between SpaceCenter.Camera.MinPitch and
    /// SpaceCenter.Camera.MaxPitch
    fn Camera_get_Pitch(this: Class) -> Float;

    /// The pitch of the camera, in degrees.
    /// A value between SpaceCenter.Camera.MinPitch and
    /// SpaceCenter.Camera.MaxPitch
    fn Camera_set_Pitch(this: Class, value: Float);

    /// The heading of the camera, in degrees.
    fn Camera_get_Heading(this: Class) -> Float;

    /// The heading of the camera, in degrees.
    fn Camera_set_Heading(this: Class, value: Float);

    /// The distance from the camera to the subject, in meters.
    /// A value between SpaceCenter.Camera.MinDistance and
    /// SpaceCenter.Camera.MaxDistance.
    fn Camera_get_Distance(this: Class) -> Float;

    /// The distance from the camera to the subject, in meters.
    /// A value between SpaceCenter.Camera.MinDistance and
    /// SpaceCenter.Camera.MaxDistance.
    fn Camera_set_Distance(this: Class, value: Float);

    /// The minimum pitch of the camera.
    fn Camera_get_MinPitch(this: Class) -> Float;

    /// The maximum pitch of the camera.
    fn Camera_get_MaxPitch(this: Class) -> Float;

    /// Minimum distance from the camera to the subject, in meters.
    fn Camera_get_MinDistance(this: Class) -> Float;

    /// Maximum distance from the camera to the subject, in meters.
    fn Camera_get_MaxDistance(this: Class) -> Float;

    /// Default distance from the camera to the subject, in meters.
    fn Camera_get_DefaultDistance(this: Class) -> Float;

    /// In map mode, the celestial body that the camera is focussed on.
    /// Returns <c>null</c> if the camera is not focussed on a celestial body.
    /// Returns an error is the camera is not in map mode.
    fn Camera_get_FocussedBody(this: Class) -> Option<Class>;

    /// In map mode, the celestial body that the camera is focussed on.
    /// Returns <c>null</c> if the camera is not focussed on a celestial body.
    /// Returns an error is the camera is not in map mode.
    fn Camera_set_FocussedBody(this: Class, value: Class);

    /// In map mode, the vessel that the camera is focussed on.
    /// Returns <c>null</c> if the camera is not focussed on a vessel.
    /// Returns an error is the camera is not in map mode.
    fn Camera_get_FocussedVessel(this: Class) -> Option<Class>;

    /// In map mode, the vessel that the camera is focussed on.
    /// Returns <c>null</c> if the camera is not focussed on a vessel.
    /// Returns an error is the camera is not in map mode.
    fn Camera_set_FocussedVessel(this: Class, value: Class);

    /// In map mode, the maneuver node that the camera is focussed on.
    /// Returns <c>null</c> if the camera is not focussed on a maneuver node.
    /// Returns an error is the camera is not in map mode.
    fn Camera_get_FocussedNode(this: Class) -> Option<Class>;

    /// In map mode, the maneuver node that the camera is focussed on.
    /// Returns <c>null</c> if the camera is not focussed on a maneuver node.
    /// Returns an error is the camera is not in map mode.
    fn Camera_set_FocussedNode(this: Class, value: Class);

    /// The height of the surface relative to mean sea level, in meters,
    /// at the given position. When over water this is equal to 0.
    /// 
    /// <param name="latitude">Latitude in degrees.</param>
    /// <param name="longitude">Longitude in degrees.</param>
    fn CelestialBody_SurfaceHeight(this: Class, latitude: Double, longitude: Double) -> Double;

    /// The height of the surface relative to mean sea level, in meters,
    /// at the given position. When over water, this is the height
    /// of the sea-bed and is therefore  negative value.
    /// 
    /// <param name="latitude">Latitude in degrees.</param>
    /// <param name="longitude">Longitude in degrees.</param>
    fn CelestialBody_BedrockHeight(this: Class, latitude: Double, longitude: Double) -> Double;

    /// The position at mean sea level at the given latitude and longitude,
    /// in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// Position as a vector.
    /// 
    /// <param name="latitude">Latitude in degrees.</param>
    /// <param name="longitude">Longitude in degrees.</param>
    /// <param name="referenceFrame">Reference frame for the returned position
    /// vector.</param>
    fn CelestialBody_MSLPosition(this: Class, latitude: Double, longitude: Double, referenceFrame: Class) -> Tuple;

    /// The position of the surface at the given latitude and longitude, in the
    /// given
    /// reference frame. When over water, this is the position of the surface
    /// of the water.
    /// 
    /// # Returns
    /// 
    /// Position as a vector.
    /// 
    /// <param name="latitude">Latitude in degrees.</param>
    /// <param name="longitude">Longitude in degrees.</param>
    /// <param name="referenceFrame">Reference frame for the returned position
    /// vector.</param>
    fn CelestialBody_SurfacePosition(this: Class, latitude: Double, longitude: Double, referenceFrame: Class) -> Tuple;

    /// The position of the surface at the given latitude and longitude, in the
    /// given
    /// reference frame. When over water, this is the position at the bottom of
    /// the sea-bed.
    /// 
    /// # Returns
    /// 
    /// Position as a vector.
    /// 
    /// <param name="latitude">Latitude in degrees.</param>
    /// <param name="longitude">Longitude in degrees.</param>
    /// <param name="referenceFrame">Reference frame for the returned position
    /// vector.</param>
    fn CelestialBody_BedrockPosition(this: Class, latitude: Double, longitude: Double, referenceFrame: Class) -> Tuple;

    /// The position at the given latitude, longitude and altitude, in the
    /// given reference frame.
    /// 
    /// # Returns
    /// 
    /// Position as a vector.
    /// 
    /// <param name="latitude">Latitude in degrees.</param>
    /// <param name="longitude">Longitude in degrees.</param>
    /// <param name="altitude">Altitude in meters above sea level.</param>
    /// <param name="referenceFrame">Reference frame for the returned position
    /// vector.</param>
    fn CelestialBody_PositionAtAltitude(this: Class, latitude: Double, longitude: Double, altitude: Double, referenceFrame: Class) -> Tuple;

    /// The latitude of the given position, in the given reference frame.
    /// 
    /// <param name="position">Position as a vector.</param>
    /// <param name="referenceFrame">Reference frame for the position
    /// vector.</param>
    fn CelestialBody_LatitudeAtPosition(this: Class, position: Tuple, referenceFrame: Class) -> Double;

    /// The longitude of the given position, in the given reference frame.
    /// 
    /// <param name="position">Position as a vector.</param>
    /// <param name="referenceFrame">Reference frame for the position
    /// vector.</param>
    fn CelestialBody_LongitudeAtPosition(this: Class, position: Tuple, referenceFrame: Class) -> Double;

    /// The altitude, in meters, of the given position in the given reference
    /// frame.
    /// 
    /// <param name="position">Position as a vector.</param>
    /// <param name="referenceFrame">Reference frame for the position
    /// vector.</param>
    fn CelestialBody_AltitudeAtPosition(this: Class, position: Tuple, referenceFrame: Class) -> Double;

    /// The atmospheric density at the given position, in <math>kg/m^3</math>,
    /// in the given reference frame.
    /// 
    /// <param name="position">The position vector at which to measure the
    /// density.</param>
    /// <param name="referenceFrame">Reference frame that the position vector
    /// is in.</param>
    fn CelestialBody_AtmosphericDensityAtPosition(this: Class, position: Tuple, referenceFrame: Class) -> Double;

    /// The temperature on the body at the given position, in the given
    /// reference frame.
    /// 
    /// <param name="position">Position as a vector.</param>
    /// <param name="referenceFrame">The reference frame that the position is
    /// in.</param>
    /// <remarks>
    /// This calculation is performed using the bodies current position, which
    /// means that
    /// the value could be wrong if you want to know the temperature in the far
    /// future.
    /// </remarks>
    fn CelestialBody_TemperatureAt(this: Class, position: Tuple, referenceFrame: Class) -> Double;

    /// Gets the air density, in <math>kg/m^3</math>, for the specified
    /// altitude above sea level, in meters.
    /// 
    /// <remarks>
    /// This is an approximation, because actual calculations, taking sun
    /// exposure into account
    /// to compute air temperature, require us to know the exact point on the
    /// body where the
    /// density is to be computed (knowing the altitude is not enough).
    /// However, the difference is small for high altitudes, so it makes very
    /// little difference
    /// for trajectory prediction.
    /// </remarks>
    fn CelestialBody_DensityAt(this: Class, altitude: Double) -> Double;

    /// Gets the air pressure, in Pascals, for the specified
    /// altitude above sea level, in meters.
    fn CelestialBody_PressureAt(this: Class, altitude: Double) -> Double;

    /// The biome at the given latitude and longitude, in degrees.
    fn CelestialBody_BiomeAt(this: Class, latitude: Double, longitude: Double) -> String;

    /// The position of the center of the body, in the specified reference
    /// frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn CelestialBody_Position(this: Class, referenceFrame: Class) -> Tuple;

    /// The linear velocity of the body, in the specified reference frame.
    /// 
    /// # Returns
    /// 
    /// The velocity as a vector. The vector points in the direction of travel,
    /// and its magnitude is the speed of the body in meters per second.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// velocity vector is in.</param>
    fn CelestialBody_Velocity(this: Class, referenceFrame: Class) -> Tuple;

    /// The rotation of the body, in the specified reference frame.
    /// 
    /// # Returns
    /// 
    /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// rotation is in.</param>
    fn CelestialBody_Rotation(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction in which the north pole of the celestial body is pointing,
    /// in the specified reference frame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn CelestialBody_Direction(this: Class, referenceFrame: Class) -> Tuple;

    /// The angular velocity of the body in the specified reference frame.
    /// 
    /// # Returns
    /// 
    /// The angular velocity as a vector. The magnitude of the vector is the
    /// rotational
    /// speed of the body, in radians per second. The direction of the vector
    /// indicates the axis
    /// of rotation, using the right-hand rule.
    /// 
    /// <param name="referenceFrame">The reference frame the returned
    /// angular velocity is in.</param>
    fn CelestialBody_AngularVelocity(this: Class, referenceFrame: Class) -> Tuple;

    /// The name of the body.
    fn CelestialBody_get_Name(this: Class) -> String;

    /// A list of celestial bodies that are in orbit around this celestial body.
    fn CelestialBody_get_Satellites(this: Class) -> List;

    /// The mass of the body, in kilograms.
    fn CelestialBody_get_Mass(this: Class) -> Float;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Standard_gravitational_parameter">standard
    /// 
    /// gravitational parameter</a> of the body in <math>m^3s^{-2}</math>.
    fn CelestialBody_get_GravitationalParameter(this: Class) -> Float;

    /// The acceleration due to gravity at sea level (mean altitude) on the
    /// body,
    /// in <math>m/s^2</math>.
    fn CelestialBody_get_SurfaceGravity(this: Class) -> Float;

    /// The sidereal rotational period of the body, in seconds.
    fn CelestialBody_get_RotationalPeriod(this: Class) -> Float;

    /// The rotational speed of the body, in radians per second.
    fn CelestialBody_get_RotationalSpeed(this: Class) -> Float;

    /// The current rotation angle of the body, in radians.
    /// A value between 0 and <math>2\pi</math>
    fn CelestialBody_get_RotationAngle(this: Class) -> Double;

    /// The initial rotation angle of the body (at UT 0), in radians.
    /// A value between 0 and <math>2\pi</math>
    fn CelestialBody_get_InitialRotation(this: Class) -> Double;

    /// The equatorial radius of the body, in meters.
    fn CelestialBody_get_EquatorialRadius(this: Class) -> Float;

    /// The radius of the sphere of influence of the body, in meters.
    fn CelestialBody_get_SphereOfInfluence(this: Class) -> Float;

    /// The orbit of the body.
    fn CelestialBody_get_Orbit(this: Class) -> Option<Class>;

    /// <c>true</c> if the body has an atmosphere.
    fn CelestialBody_get_HasAtmosphere(this: Class) -> Bool;

    /// The depth of the atmosphere, in meters.
    fn CelestialBody_get_AtmosphereDepth(this: Class) -> Float;

    /// <c>true</c> if there is oxygen in the atmosphere, required for
    /// air-breathing engines.
    fn CelestialBody_get_HasAtmosphericOxygen(this: Class) -> Bool;

    /// The biomes present on this body.
    fn CelestialBody_get_Biomes(this: Class) -> Set;

    /// The altitude, in meters, above which a vessel is considered to be
    /// flying "high" when doing science.
    fn CelestialBody_get_FlyingHighAltitudeThreshold(this: Class) -> Float;

    /// The altitude, in meters, above which a vessel is considered to be
    /// in "high" space when doing science.
    fn CelestialBody_get_SpaceHighAltitudeThreshold(this: Class) -> Float;

    /// The reference frame that is fixed relative to the celestial body.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// the body.
    /// </description></item><item><description>The axes rotate with the
    /// body.</description></item><item><description>The x-axis points from the
    /// center of the body
    /// towards the intersection of the prime meridian and equator (the
    /// position at 0° longitude, 0°
    /// latitude).</description></item><item><description>The y-axis points
    /// from the center of the body
    /// towards the north pole.</description></item><item><description>The
    /// z-axis points from the center of the body
    /// towards the equator at 90°E longitude.</description></item></list>
    fn CelestialBody_get_ReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to this celestial body, and
    /// orientated in a fixed direction (it does not rotate with the body).
    /// <list type="bullet"><item><description>The origin is at the center of
    /// the body.</description></item><item><description>The axes do not
    /// rotate.</description></item><item><description>The x-axis points in an
    /// arbitrary direction through the
    /// equator.</description></item><item><description>The y-axis points from
    /// the center of the body towards
    /// the north pole.</description></item><item><description>The z-axis
    /// points in an arbitrary direction through the
    /// equator.</description></item></list>
    fn CelestialBody_get_NonRotatingReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to this celestial body, but
    /// orientated with the body's orbital prograde/normal/radial directions.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// the body.
    /// </description></item><item><description>The axes rotate with the
    /// orbital prograde/normal/radial
    /// directions.</description></item><item><description>The x-axis points in
    /// the orbital anti-radial direction.
    /// </description></item><item><description>The y-axis points in the
    /// orbital prograde direction.
    /// </description></item><item><description>The z-axis points in the
    /// orbital normal direction.
    /// </description></item></list>
    fn CelestialBody_get_OrbitalReferenceFrame(this: Class) -> Class;

    /// The type of link.
    fn CommLink_get_Type(this: Class) -> Enumeration;

    /// Signal strength of the link.
    fn CommLink_get_SignalStrength(this: Class) -> Double;

    /// Start point of the link.
    fn CommLink_get_Start(this: Class) -> Class;

    /// Start point of the link.
    fn CommLink_get_End(this: Class) -> Class;

    /// Name of the communication node.
    fn CommNode_get_Name(this: Class) -> String;

    /// Whether the communication node is on Kerbin.
    fn CommNode_get_IsHome(this: Class) -> Bool;

    /// Whether the communication node is a control point, for example a manned
    /// vessel.
    fn CommNode_get_IsControlPoint(this: Class) -> Bool;

    /// Whether the communication node is a vessel.
    fn CommNode_get_IsVessel(this: Class) -> Bool;

    /// The vessel for this communication node.
    fn CommNode_get_Vessel(this: Class) -> Class;

    /// Whether the vessel can communicate with KSC.
    fn Comms_get_CanCommunicate(this: Class) -> Bool;

    /// Whether the vessel can transmit science data to KSC.
    fn Comms_get_CanTransmitScience(this: Class) -> Bool;

    /// Signal strength to KSC.
    fn Comms_get_SignalStrength(this: Class) -> Double;

    /// Signal delay to KSC in seconds.
    fn Comms_get_SignalDelay(this: Class) -> Double;

    /// The combined power of all active antennae on the vessel.
    fn Comms_get_Power(this: Class) -> Double;

    /// The communication path used to control the vessel.
    fn Comms_get_ControlPath(this: Class) -> List;

    /// Cancel an active contract.
    fn Contract_Cancel(this: Class);

    /// Accept an offered contract.
    fn Contract_Accept(this: Class);

    /// Decline an offered contract.
    fn Contract_Decline(this: Class);

    /// Type of the contract.
    fn Contract_get_Type(this: Class) -> String;

    /// Title of the contract.
    fn Contract_get_Title(this: Class) -> String;

    /// Description of the contract.
    fn Contract_get_Description(this: Class) -> String;

    /// Notes for the contract.
    fn Contract_get_Notes(this: Class) -> String;

    /// Synopsis for the contract.
    fn Contract_get_Synopsis(this: Class) -> String;

    /// Keywords for the contract.
    fn Contract_get_Keywords(this: Class) -> List;

    /// State of the contract.
    fn Contract_get_State(this: Class) -> Enumeration;

    /// Whether the contract is active.
    fn Contract_get_Active(this: Class) -> Bool;

    /// Whether the contract has been failed.
    fn Contract_get_Failed(this: Class) -> Bool;

    /// Whether the contract has been seen.
    fn Contract_get_Seen(this: Class) -> Bool;

    /// Whether the contract has been read.
    fn Contract_get_Read(this: Class) -> Bool;

    /// Whether the contract can be canceled.
    fn Contract_get_CanBeCanceled(this: Class) -> Bool;

    /// Whether the contract can be declined.
    fn Contract_get_CanBeDeclined(this: Class) -> Bool;

    /// Whether the contract can be failed.
    fn Contract_get_CanBeFailed(this: Class) -> Bool;

    /// Funds received when accepting the contract.
    fn Contract_get_FundsAdvance(this: Class) -> Double;

    /// Funds received on completion of the contract.
    fn Contract_get_FundsCompletion(this: Class) -> Double;

    /// Funds lost if the contract is failed.
    fn Contract_get_FundsFailure(this: Class) -> Double;

    /// Reputation gained on completion of the contract.
    fn Contract_get_ReputationCompletion(this: Class) -> Double;

    /// Reputation lost if the contract is failed.
    fn Contract_get_ReputationFailure(this: Class) -> Double;

    /// Science gained on completion of the contract.
    fn Contract_get_ScienceCompletion(this: Class) -> Double;

    /// Parameters for the contract.
    fn Contract_get_Parameters(this: Class) -> List;

    /// A list of all contract types.
    fn ContractManager_get_Types(this: Class) -> Set;

    /// A list of all contracts.
    fn ContractManager_get_AllContracts(this: Class) -> List;

    /// A list of all active contracts.
    fn ContractManager_get_ActiveContracts(this: Class) -> List;

    /// A list of all offered, but unaccepted, contracts.
    fn ContractManager_get_OfferedContracts(this: Class) -> List;

    /// A list of all completed contracts.
    fn ContractManager_get_CompletedContracts(this: Class) -> List;

    /// A list of all failed contracts.
    fn ContractManager_get_FailedContracts(this: Class) -> List;

    /// Title of the parameter.
    fn ContractParameter_get_Title(this: Class) -> String;

    /// Notes for the parameter.
    fn ContractParameter_get_Notes(this: Class) -> String;

    /// Child contract parameters.
    fn ContractParameter_get_Children(this: Class) -> List;

    /// Whether the parameter has been completed.
    fn ContractParameter_get_Completed(this: Class) -> Bool;

    /// Whether the parameter has been failed.
    fn ContractParameter_get_Failed(this: Class) -> Bool;

    /// Whether the contract parameter is optional.
    fn ContractParameter_get_Optional(this: Class) -> Bool;

    /// Funds received on completion of the contract parameter.
    fn ContractParameter_get_FundsCompletion(this: Class) -> Double;

    /// Funds lost if the contract parameter is failed.
    fn ContractParameter_get_FundsFailure(this: Class) -> Double;

    /// Reputation gained on completion of the contract parameter.
    fn ContractParameter_get_ReputationCompletion(this: Class) -> Double;

    /// Reputation lost if the contract parameter is failed.
    fn ContractParameter_get_ReputationFailure(this: Class) -> Double;

    /// Science gained on completion of the contract parameter.
    fn ContractParameter_get_ScienceCompletion(this: Class) -> Double;

    /// Activates the next stage. Equivalent to pressing the space bar in-game.
    /// 
    /// # Returns
    /// 
    /// A list of vessel objects that are jettisoned from the active vessel.
    /// 
    /// <remarks>
    /// When called, the active vessel may change. It is therefore possible
    /// that,
    /// after calling this function, the object(s) returned by previous call(s)
    /// to
    /// SpaceCenter.ActiveVessel no longer refer to the active vessel.
    /// </remarks>
    fn Control_ActivateNextStage(this: Class) -> List;

    /// Returns <c>true</c> if the given action group is enabled.
    /// 
    /// <param name="group">
    /// A number between 0 and 9 inclusive,
    /// or between 0 and 250 inclusive when the <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed.
    /// 
    /// </param>
    fn Control_GetActionGroup(this: Class, group: Uint32) -> Bool;

    /// Sets the state of the given action group.
    /// 
    /// <param name="group">
    /// A number between 0 and 9 inclusive,
    /// or between 0 and 250 inclusive when the <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed.
    /// 
    /// </param>
    /// <param name="state"></param>
    fn Control_SetActionGroup(this: Class, group: Uint32, state: Bool);

    /// Toggles the state of the given action group.
    /// 
    /// <param name="group">
    /// A number between 0 and 9 inclusive,
    /// or between 0 and 250 inclusive when the <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed.
    /// 
    /// </param>
    fn Control_ToggleActionGroup(this: Class, group: Uint32);

    /// Creates a maneuver node at the given universal time, and returns a
    /// <see cref="T:SpaceCenter.Node object that can be used to modify it.
    /// Optionally sets the magnitude of the delta-v for the maneuver node
    /// in the prograde, normal and radial directions.
    /// 
    /// <param name="ut">Universal time of the maneuver node.</param>
    /// <param name="prograde">Delta-v in the prograde direction.</param>
    /// <param name="normal">Delta-v in the normal direction.</param>
    /// <param name="radial">Delta-v in the radial direction.</param>
    fn Control_AddNode(this: Class, ut: Double, prograde: Float, normal: Float, radial: Float) -> Class;

    /// Remove all maneuver nodes.
    fn Control_RemoveNodes(this: Class);

    /// The control state of the vessel.
    fn Control_get_State(this: Class) -> Enumeration;

    /// The source of the vessels control, for example by a kerbal or a probe
    /// core.
    fn Control_get_Source(this: Class) -> Enumeration;

    /// The state of SAS.
    /// 
    /// <remarks>Equivalent to SpaceCenter.AutoPilot.SAS</remarks>
    fn Control_get_SAS(this: Class) -> Bool;

    /// The state of SAS.
    /// 
    /// <remarks>Equivalent to SpaceCenter.AutoPilot.SAS</remarks>
    fn Control_set_SAS(this: Class, value: Bool);

    /// The current <see cref="T:SpaceCenter.SASMode.
    /// These modes are equivalent to the mode buttons to
    /// the left of the navball that appear when SAS is enabled.
    /// 
    /// <remarks>Equivalent to SpaceCenter.AutoPilot.SASMode</remarks>
    fn Control_get_SASMode(this: Class) -> Enumeration;

    /// The current <see cref="T:SpaceCenter.SASMode.
    /// These modes are equivalent to the mode buttons to
    /// the left of the navball that appear when SAS is enabled.
    /// 
    /// <remarks>Equivalent to SpaceCenter.AutoPilot.SASMode</remarks>
    fn Control_set_SASMode(this: Class, value: Enumeration);

    /// The current <see cref="T:SpaceCenter.SpeedMode of the navball.
    /// This is the mode displayed next to the speed at the top of the navball.
    fn Control_get_SpeedMode(this: Class) -> Enumeration;

    /// The current <see cref="T:SpaceCenter.SpeedMode of the navball.
    /// This is the mode displayed next to the speed at the top of the navball.
    fn Control_set_SpeedMode(this: Class, value: Enumeration);

    /// The state of RCS.
    fn Control_get_RCS(this: Class) -> Bool;

    /// The state of RCS.
    fn Control_set_RCS(this: Class, value: Bool);

    /// Returns whether all reactive wheels on the vessel are active,
    /// and sets the active state of all reaction wheels.
    /// See SpaceCenter.ReactionWheel.Active.
    fn Control_get_ReactionWheels(this: Class) -> Bool;

    /// Returns whether all reactive wheels on the vessel are active,
    /// and sets the active state of all reaction wheels.
    /// See SpaceCenter.ReactionWheel.Active.
    fn Control_set_ReactionWheels(this: Class, value: Bool);

    /// The state of the landing gear/legs.
    fn Control_get_Gear(this: Class) -> Bool;

    /// The state of the landing gear/legs.
    fn Control_set_Gear(this: Class, value: Bool);

    /// Returns whether all landing legs on the vessel are deployed,
    /// and sets the deployment state of all landing legs.
    /// Does not include wheels (for example landing gear).
    /// See SpaceCenter.Leg.Deployed.
    fn Control_get_Legs(this: Class) -> Bool;

    /// Returns whether all landing legs on the vessel are deployed,
    /// and sets the deployment state of all landing legs.
    /// Does not include wheels (for example landing gear).
    /// See SpaceCenter.Leg.Deployed.
    fn Control_set_Legs(this: Class, value: Bool);

    /// Returns whether all wheels on the vessel are deployed,
    /// and sets the deployment state of all wheels.
    /// Does not include landing legs.
    /// See SpaceCenter.Wheel.Deployed.
    fn Control_get_Wheels(this: Class) -> Bool;

    /// Returns whether all wheels on the vessel are deployed,
    /// and sets the deployment state of all wheels.
    /// Does not include landing legs.
    /// See SpaceCenter.Wheel.Deployed.
    fn Control_set_Wheels(this: Class, value: Bool);

    /// The state of the lights.
    fn Control_get_Lights(this: Class) -> Bool;

    /// The state of the lights.
    fn Control_set_Lights(this: Class, value: Bool);

    /// The state of the wheel brakes.
    fn Control_get_Brakes(this: Class) -> Bool;

    /// The state of the wheel brakes.
    fn Control_set_Brakes(this: Class, value: Bool);

    /// Returns whether all antennas on the vessel are deployed,
    /// and sets the deployment state of all antennas.
    /// See SpaceCenter.Antenna.Deployed.
    fn Control_get_Antennas(this: Class) -> Bool;

    /// Returns whether all antennas on the vessel are deployed,
    /// and sets the deployment state of all antennas.
    /// See SpaceCenter.Antenna.Deployed.
    fn Control_set_Antennas(this: Class, value: Bool);

    /// Returns whether any of the cargo bays on the vessel are open,
    /// and sets the open state of all cargo bays.
    /// See SpaceCenter.CargoBay.Open.
    fn Control_get_CargoBays(this: Class) -> Bool;

    /// Returns whether any of the cargo bays on the vessel are open,
    /// and sets the open state of all cargo bays.
    /// See SpaceCenter.CargoBay.Open.
    fn Control_set_CargoBays(this: Class, value: Bool);

    /// Returns whether all of the air intakes on the vessel are open,
    /// and sets the open state of all air intakes.
    /// See SpaceCenter.Intake.Open.
    fn Control_get_Intakes(this: Class) -> Bool;

    /// Returns whether all of the air intakes on the vessel are open,
    /// and sets the open state of all air intakes.
    /// See SpaceCenter.Intake.Open.
    fn Control_set_Intakes(this: Class, value: Bool);

    /// Returns whether all parachutes on the vessel are deployed,
    /// and sets the deployment state of all parachutes.
    /// Cannot be set to <c>false</c>.
    /// See SpaceCenter.Parachute.Deployed.
    fn Control_get_Parachutes(this: Class) -> Bool;

    /// Returns whether all parachutes on the vessel are deployed,
    /// and sets the deployment state of all parachutes.
    /// Cannot be set to <c>false</c>.
    /// See SpaceCenter.Parachute.Deployed.
    fn Control_set_Parachutes(this: Class, value: Bool);

    /// Returns whether all radiators on the vessel are deployed,
    /// and sets the deployment state of all radiators.
    /// See SpaceCenter.Radiator.Deployed.
    fn Control_get_Radiators(this: Class) -> Bool;

    /// Returns whether all radiators on the vessel are deployed,
    /// and sets the deployment state of all radiators.
    /// See SpaceCenter.Radiator.Deployed.
    fn Control_set_Radiators(this: Class, value: Bool);

    /// Returns whether all of the resource harvesters on the vessel are
    /// deployed,
    /// and sets the deployment state of all resource harvesters.
    /// See SpaceCenter.ResourceHarvester.Deployed.
    fn Control_get_ResourceHarvesters(this: Class) -> Bool;

    /// Returns whether all of the resource harvesters on the vessel are
    /// deployed,
    /// and sets the deployment state of all resource harvesters.
    /// See SpaceCenter.ResourceHarvester.Deployed.
    fn Control_set_ResourceHarvesters(this: Class, value: Bool);

    /// Returns whether any of the resource harvesters on the vessel are active,
    /// and sets the active state of all resource harvesters.
    /// See SpaceCenter.ResourceHarvester.Active.
    fn Control_get_ResourceHarvestersActive(this: Class) -> Bool;

    /// Returns whether any of the resource harvesters on the vessel are active,
    /// and sets the active state of all resource harvesters.
    /// See SpaceCenter.ResourceHarvester.Active.
    fn Control_set_ResourceHarvestersActive(this: Class, value: Bool);

    /// Returns whether all solar panels on the vessel are deployed,
    /// and sets the deployment state of all solar panels.
    /// See SpaceCenter.SolarPanel.Deployed.
    fn Control_get_SolarPanels(this: Class) -> Bool;

    /// Returns whether all solar panels on the vessel are deployed,
    /// and sets the deployment state of all solar panels.
    /// See SpaceCenter.SolarPanel.Deployed.
    fn Control_set_SolarPanels(this: Class, value: Bool);

    /// The state of the abort action group.
    fn Control_get_Abort(this: Class) -> Bool;

    /// The state of the abort action group.
    fn Control_set_Abort(this: Class, value: Bool);

    /// The state of the throttle. A value between 0 and 1.
    fn Control_get_Throttle(this: Class) -> Float;

    /// The state of the throttle. A value between 0 and 1.
    fn Control_set_Throttle(this: Class, value: Float);

    /// Sets the behavior of the pitch, yaw, roll and translation control
    /// inputs.
    /// When set to additive, these inputs are added to the vessels current
    /// inputs.
    /// This mode is the default.
    /// When set to override, these inputs (if non-zero) override the vessels
    /// inputs.
    /// This mode prevents keyboard control, or SAS, from interfering with the
    /// controls when
    /// they are set.
    fn Control_get_InputMode(this: Class) -> Enumeration;

    /// Sets the behavior of the pitch, yaw, roll and translation control
    /// inputs.
    /// When set to additive, these inputs are added to the vessels current
    /// inputs.
    /// This mode is the default.
    /// When set to override, these inputs (if non-zero) override the vessels
    /// inputs.
    /// This mode prevents keyboard control, or SAS, from interfering with the
    /// controls when
    /// they are set.
    fn Control_set_InputMode(this: Class, value: Enumeration);

    /// The state of the pitch control.
    /// A value between -1 and 1.
    /// Equivalent to the w and s keys.
    fn Control_get_Pitch(this: Class) -> Float;

    /// The state of the pitch control.
    /// A value between -1 and 1.
    /// Equivalent to the w and s keys.
    fn Control_set_Pitch(this: Class, value: Float);

    /// The state of the yaw control.
    /// A value between -1 and 1.
    /// Equivalent to the a and d keys.
    fn Control_get_Yaw(this: Class) -> Float;

    /// The state of the yaw control.
    /// A value between -1 and 1.
    /// Equivalent to the a and d keys.
    fn Control_set_Yaw(this: Class, value: Float);

    /// The state of the roll control.
    /// A value between -1 and 1.
    /// Equivalent to the q and e keys.
    fn Control_get_Roll(this: Class) -> Float;

    /// The state of the roll control.
    /// A value between -1 and 1.
    /// Equivalent to the q and e keys.
    fn Control_set_Roll(this: Class, value: Float);

    /// The state of the forward translational control.
    /// A value between -1 and 1.
    /// Equivalent to the h and n keys.
    fn Control_get_Forward(this: Class) -> Float;

    /// The state of the forward translational control.
    /// A value between -1 and 1.
    /// Equivalent to the h and n keys.
    fn Control_set_Forward(this: Class, value: Float);

    /// The state of the up translational control.
    /// A value between -1 and 1.
    /// Equivalent to the i and k keys.
    fn Control_get_Up(this: Class) -> Float;

    /// The state of the up translational control.
    /// A value between -1 and 1.
    /// Equivalent to the i and k keys.
    fn Control_set_Up(this: Class, value: Float);

    /// The state of the right translational control.
    /// A value between -1 and 1.
    /// Equivalent to the j and l keys.
    fn Control_get_Right(this: Class) -> Float;

    /// The state of the right translational control.
    /// A value between -1 and 1.
    /// Equivalent to the j and l keys.
    fn Control_set_Right(this: Class, value: Float);

    /// The state of the wheel throttle.
    /// A value between -1 and 1.
    /// A value of 1 rotates the wheels forwards, a value of -1 rotates
    /// the wheels backwards.
    fn Control_get_WheelThrottle(this: Class) -> Float;

    /// The state of the wheel throttle.
    /// A value between -1 and 1.
    /// A value of 1 rotates the wheels forwards, a value of -1 rotates
    /// the wheels backwards.
    fn Control_set_WheelThrottle(this: Class, value: Float);

    /// The state of the wheel steering.
    /// A value between -1 and 1.
    /// A value of 1 steers to the left, and a value of -1 steers to the right.
    fn Control_get_WheelSteering(this: Class) -> Float;

    /// The state of the wheel steering.
    /// A value between -1 and 1.
    /// A value of 1 steers to the left, and a value of -1 steers to the right.
    fn Control_set_WheelSteering(this: Class, value: Float);

    /// The current stage of the vessel. Corresponds to the stage number in
    /// the in-game UI.
    fn Control_get_CurrentStage(this: Class) -> Sint32;

    /// Returns a list of all existing maneuver nodes, ordered by time from
    /// first to last.
    fn Control_get_Nodes(this: Class) -> List;

    /// The crew members name.
    fn CrewMember_get_Name(this: Class) -> String;

    /// The crew members name.
    fn CrewMember_set_Name(this: Class, value: String);

    /// The type of crew member.
    fn CrewMember_get_Type(this: Class) -> Enumeration;

    /// Whether the crew member is on a mission.
    fn CrewMember_get_OnMission(this: Class) -> Bool;

    /// The crew members courage.
    fn CrewMember_get_Courage(this: Class) -> Float;

    /// The crew members courage.
    fn CrewMember_set_Courage(this: Class, value: Float);

    /// The crew members stupidity.
    fn CrewMember_get_Stupidity(this: Class) -> Float;

    /// The crew members stupidity.
    fn CrewMember_set_Stupidity(this: Class, value: Float);

    /// The crew members experience.
    fn CrewMember_get_Experience(this: Class) -> Float;

    /// The crew members experience.
    fn CrewMember_set_Experience(this: Class, value: Float);

    /// Whether the crew member is a badass.
    fn CrewMember_get_Badass(this: Class) -> Bool;

    /// Whether the crew member is a badass.
    fn CrewMember_set_Badass(this: Class, value: Bool);

    /// Whether the crew member is a veteran.
    fn CrewMember_get_Veteran(this: Class) -> Bool;

    /// Whether the crew member is a veteran.
    fn CrewMember_set_Veteran(this: Class, value: Bool);

    /// Simulate and return the total aerodynamic forces acting on the vessel,
    /// if it where to be traveling with the given velocity at the given
    /// position in the
    /// atmosphere of the given celestial body.
    /// 
    /// # Returns
    /// 
    /// A vector pointing in the direction that the force acts,
    /// with its magnitude equal to the strength of the force in Newtons.
    fn Flight_SimulateAerodynamicForceAt(this: Class, body: Class, position: Tuple, velocity: Tuple) -> Tuple;

    /// The current G force acting on the vessel in <math>g</math>.
    fn Flight_get_GForce(this: Class) -> Float;

    /// The altitude above sea level, in meters.
    /// Measured from the center of mass of the vessel.
    fn Flight_get_MeanAltitude(this: Class) -> Double;

    /// The altitude above the surface of the body or sea level, whichever is
    /// closer, in meters.
    /// Measured from the center of mass of the vessel.
    fn Flight_get_SurfaceAltitude(this: Class) -> Double;

    /// The altitude above the surface of the body, in meters. When over water,
    /// this is the altitude above the sea floor.
    /// Measured from the center of mass of the vessel.
    fn Flight_get_BedrockAltitude(this: Class) -> Double;

    /// The elevation of the terrain under the vessel, in meters. This is the
    /// height of the terrain above sea level,
    /// and is negative when the vessel is over the sea.
    fn Flight_get_Elevation(this: Class) -> Double;

    /// The <a href="https://en.wikipedia.org/wiki/Latitude">latitude</a> of
    /// the vessel for the body being orbited, in degrees.
    fn Flight_get_Latitude(this: Class) -> Double;

    /// The <a href="https://en.wikipedia.org/wiki/Longitude">longitude</a> of
    /// the vessel for the body being orbited, in degrees.
    fn Flight_get_Longitude(this: Class) -> Double;

    /// The velocity of the vessel, in the reference frame <see
    /// cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The velocity as a vector. The vector points in the direction of travel,
    /// and its magnitude is the speed of the vessel in meters per second.
    fn Flight_get_Velocity(this: Class) -> Tuple;

    /// The speed of the vessel in meters per second,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    fn Flight_get_Speed(this: Class) -> Double;

    /// The horizontal speed of the vessel in meters per second,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    fn Flight_get_HorizontalSpeed(this: Class) -> Double;

    /// The vertical speed of the vessel in meters per second,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    fn Flight_get_VerticalSpeed(this: Class) -> Double;

    /// The position of the center of mass of the vessel,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame
    /// # Returns
    /// 
    /// The position as a vector.
    fn Flight_get_CenterOfMass(this: Class) -> Tuple;

    /// The rotation of the vessel, in the reference frame <see
    /// cref="T:SpaceCenter.ReferenceFrame
    /// # Returns
    /// 
    /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
    fn Flight_get_Rotation(this: Class) -> Tuple;

    /// The direction that the vessel is pointing in,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_Direction(this: Class) -> Tuple;

    /// The pitch of the vessel relative to the horizon, in degrees.
    /// A value between -90° and +90°.
    fn Flight_get_Pitch(this: Class) -> Float;

    /// The heading of the vessel (its angle relative to north), in degrees.
    /// A value between 0° and 360°.
    fn Flight_get_Heading(this: Class) -> Float;

    /// The roll of the vessel relative to the horizon, in degrees.
    /// A value between -180° and +180°.
    fn Flight_get_Roll(this: Class) -> Float;

    /// The prograde direction of the vessels orbit,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_Prograde(this: Class) -> Tuple;

    /// The retrograde direction of the vessels orbit,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_Retrograde(this: Class) -> Tuple;

    /// The direction normal to the vessels orbit,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_Normal(this: Class) -> Tuple;

    /// The direction opposite to the normal of the vessels orbit,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_AntiNormal(this: Class) -> Tuple;

    /// The radial direction of the vessels orbit,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_Radial(this: Class) -> Tuple;

    /// The direction opposite to the radial direction of the vessels orbit,
    /// in the reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    fn Flight_get_AntiRadial(this: Class) -> Tuple;

    /// The current density of the atmosphere around the vessel, in
    /// <math>kg/m^3</math>.
    fn Flight_get_AtmosphereDensity(this: Class) -> Float;

    /// The dynamic pressure acting on the vessel, in Pascals. This is a
    /// measure of the
    /// strength of the aerodynamic forces. It is equal to
    /// <math>\frac{1}{2} . \mbox{air density} . \mbox{velocity}^2</math>.
    /// It is commonly denoted <math>Q</math>.
    fn Flight_get_DynamicPressure(this: Class) -> Float;

    /// The static atmospheric pressure at mean sea level, in Pascals.
    fn Flight_get_StaticPressureAtMSL(this: Class) -> Float;

    /// The static atmospheric pressure acting on the vessel, in Pascals.
    fn Flight_get_StaticPressure(this: Class) -> Float;

    /// The total aerodynamic forces acting on the vessel,
    /// in reference frame <see cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// A vector pointing in the direction that the force acts,
    /// with its magnitude equal to the strength of the force in Newtons.
    fn Flight_get_AerodynamicForce(this: Class) -> Tuple;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Aerodynamic_force">aerodynamic
    /// lift</a>
    /// currently acting on the vessel.
    /// 
    /// # Returns
    /// 
    /// A vector pointing in the direction that the force acts,
    /// with its magnitude equal to the strength of the force in Newtons.
    fn Flight_get_Lift(this: Class) -> Tuple;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Aerodynamic_force">aerodynamic
    /// drag</a> currently acting on the vessel.
    /// 
    /// # Returns
    /// 
    /// A vector pointing in the direction of the force, with its magnitude
    /// equal to the strength of the force in Newtons.
    fn Flight_get_Drag(this: Class) -> Tuple;

    /// The speed of sound, in the atmosphere around the vessel, in
    /// <math>m/s</math>.
    fn Flight_get_SpeedOfSound(this: Class) -> Float;

    /// The speed of the vessel, in multiples of the speed of sound.
    fn Flight_get_Mach(this: Class) -> Float;

    /// The vessels Reynolds number.
    /// 
    /// <remarks>
    /// Requires <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
    /// 
    /// </remarks>
    fn Flight_get_ReynoldsNumber(this: Class) -> Float;

    /// The <a href="https://en.wikipedia.org/wiki/True_airspeed">true air
    /// speed</a>
    /// of the vessel, in meters per second.
    fn Flight_get_TrueAirSpeed(this: Class) -> Float;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Equivalent_airspeed">equivalent air
    /// speed</a>
    /// of the vessel, in meters per second.
    fn Flight_get_EquivalentAirSpeed(this: Class) -> Float;

    /// An estimate of the current terminal velocity of the vessel, in meters
    /// per second.
    /// This is the speed at which the drag forces cancel out the force of
    /// gravity.
    fn Flight_get_TerminalVelocity(this: Class) -> Float;

    /// The pitch angle between the orientation of the vessel and its velocity
    /// vector,
    /// in degrees.
    fn Flight_get_AngleOfAttack(this: Class) -> Float;

    /// The yaw angle between the orientation of the vessel and its velocity
    /// vector, in degrees.
    fn Flight_get_SideslipAngle(this: Class) -> Float;

    /// The <a href="https://en.wikipedia.org/wiki/Total_air_temperature">total
    /// air temperature</a>
    /// of the atmosphere around the vessel, in Kelvin.
    /// This includes the SpaceCenter.Flight.StaticAirTemperature and the
    /// vessel's kinetic energy.
    fn Flight_get_TotalAirTemperature(this: Class) -> Float;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Total_air_temperature">static
    /// (ambient)
    /// temperature</a> of the atmosphere around the vessel, in Kelvin.
    fn Flight_get_StaticAirTemperature(this: Class) -> Float;

    /// The current amount of stall, between 0 and 1. A value greater than
    /// 0.005 indicates
    /// a minor stall and a value greater than 0.5 indicates a large-scale
    /// stall.
    /// 
    /// <remarks>
    /// Requires <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
    /// 
    /// </remarks>
    fn Flight_get_StallFraction(this: Class) -> Float;

    /// The coefficient of drag. This is the amount of drag produced by the
    /// vessel.
    /// It depends on air speed, air density and wing area.
    /// 
    /// <remarks>
    /// Requires <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
    /// 
    /// </remarks>
    fn Flight_get_DragCoefficient(this: Class) -> Float;

    /// The coefficient of lift. This is the amount of lift produced by the
    /// vessel, and
    /// depends on air speed, air density and wing area.
    /// 
    /// <remarks>
    /// Requires <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
    /// 
    /// </remarks>
    fn Flight_get_LiftCoefficient(this: Class) -> Float;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Ballistic_coefficient">ballistic
    /// coefficient</a>.
    /// 
    /// <remarks>
    /// Requires <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
    /// 
    /// </remarks>
    fn Flight_get_BallisticCoefficient(this: Class) -> Float;

    /// The thrust specific fuel consumption for the jet engines on the vessel.
    /// This is a
    /// measure of the efficiency of the engines, with a lower value indicating
    /// a more
    /// efficient vessel. This value is the number of Newtons of fuel that are
    /// burned,
    /// per hour, to produce one newton of thrust.
    /// 
    /// <remarks>
    /// Requires <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
    /// 
    /// </remarks>
    fn Flight_get_ThrustSpecificFuelConsumption(this: Class) -> Float;

    /// Returns the burn vector for the maneuver node.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// vector is in.
    /// Defaults to SpaceCenter.Vessel.OrbitalReferenceFrame.</param>
    /// # Returns
    /// 
    /// A vector whose direction is the direction of the maneuver node burn, and
    /// magnitude is the delta-v of the burn in meters per second.
    /// 
    /// 
    /// <remarks>
    /// Does not change when executing the maneuver node. See
    /// SpaceCenter.Node.RemainingBurnVector.
    /// </remarks>
    fn Node_BurnVector(this: Class, referenceFrame: Class) -> Tuple;

    /// Returns the remaining burn vector for the maneuver node.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// vector is in.
    /// Defaults to SpaceCenter.Vessel.OrbitalReferenceFrame.</param>
    /// # Returns
    /// 
    /// A vector whose direction is the direction of the maneuver node burn, and
    /// magnitude is the delta-v of the burn in meters per second.
    /// 
    /// 
    /// <remarks>
    /// Changes as the maneuver node is executed. See
    /// SpaceCenter.Node.BurnVector.
    /// </remarks>
    fn Node_RemainingBurnVector(this: Class, referenceFrame: Class) -> Tuple;

    /// Removes the maneuver node.
    fn Node_Remove(this: Class);

    /// The position vector of the maneuver node in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn Node_Position(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction of the maneuver nodes burn.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Node_Direction(this: Class, referenceFrame: Class) -> Tuple;

    /// The magnitude of the maneuver nodes delta-v in the prograde direction,
    /// in meters per second.
    fn Node_get_Prograde(this: Class) -> Double;

    /// The magnitude of the maneuver nodes delta-v in the prograde direction,
    /// in meters per second.
    fn Node_set_Prograde(this: Class, value: Double);

    /// The magnitude of the maneuver nodes delta-v in the normal direction,
    /// in meters per second.
    fn Node_get_Normal(this: Class) -> Double;

    /// The magnitude of the maneuver nodes delta-v in the normal direction,
    /// in meters per second.
    fn Node_set_Normal(this: Class, value: Double);

    /// The magnitude of the maneuver nodes delta-v in the radial direction,
    /// in meters per second.
    fn Node_get_Radial(this: Class) -> Double;

    /// The magnitude of the maneuver nodes delta-v in the radial direction,
    /// in meters per second.
    fn Node_set_Radial(this: Class, value: Double);

    /// The delta-v of the maneuver node, in meters per second.
    /// 
    /// <remarks>
    /// Does not change when executing the maneuver node. See
    /// SpaceCenter.Node.RemainingDeltaV.
    /// </remarks>
    fn Node_get_DeltaV(this: Class) -> Double;

    /// The delta-v of the maneuver node, in meters per second.
    /// 
    /// <remarks>
    /// Does not change when executing the maneuver node. See
    /// SpaceCenter.Node.RemainingDeltaV.
    /// </remarks>
    fn Node_set_DeltaV(this: Class, value: Double);

    /// Gets the remaining delta-v of the maneuver node, in meters per second.
    /// Changes as the
    /// node is executed. This is equivalent to the delta-v reported in-game.
    fn Node_get_RemainingDeltaV(this: Class) -> Double;

    /// The universal time at which the maneuver will occur, in seconds.
    fn Node_get_UT(this: Class) -> Double;

    /// The universal time at which the maneuver will occur, in seconds.
    fn Node_set_UT(this: Class, value: Double);

    /// The time until the maneuver node will be encountered, in seconds.
    fn Node_get_TimeTo(this: Class) -> Double;

    /// The orbit that results from executing the maneuver node.
    fn Node_get_Orbit(this: Class) -> Class;

    /// The reference frame that is fixed relative to the maneuver node's burn.
    /// <list type="bullet"><item><description>The origin is at the position of
    /// the maneuver node.</description></item><item><description>The y-axis
    /// points in the direction of the
    /// burn.</description></item><item><description>The x-axis and z-axis
    /// point in arbitrary but fixed directions.</description></item></list>
    fn Node_get_ReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to the maneuver node, and
    /// orientated with the orbital prograde/normal/radial directions of the
    /// original orbit at the maneuver node's position.
    /// <list type="bullet"><item><description>The origin is at the position of
    /// the maneuver node.</description></item><item><description>The x-axis
    /// points in the orbital anti-radial direction of the original
    /// orbit, at the position of the maneuver
    /// node.</description></item><item><description>The y-axis points in the
    /// orbital prograde direction of the original
    /// orbit, at the position of the maneuver
    /// node.</description></item><item><description>The z-axis points in the
    /// orbital normal direction of the original orbit,
    /// at the position of the maneuver node.</description></item></list>
    fn Node_get_OrbitalReferenceFrame(this: Class) -> Class;

    /// The direction that is normal to the orbits reference plane,
    /// in the given reference frame.
    /// The reference plane is the plane from which the orbits inclination is
    /// measured.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Orbit_static_ReferencePlaneNormal(referenceFrame: Class) -> Tuple;

    /// The direction from which the orbits longitude of ascending node is
    /// measured,
    /// in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Orbit_static_ReferencePlaneDirection(referenceFrame: Class) -> Tuple;

    /// The mean anomaly at the given time.
    /// 
    /// <param name="ut">The universal time in seconds.</param>
    fn Orbit_MeanAnomalyAtUT(this: Class, ut: Double) -> Double;

    /// The orbital radius at the point in the orbit given by the true anomaly.
    /// 
    /// <param name="trueAnomaly">The true anomaly.</param>
    fn Orbit_RadiusAtTrueAnomaly(this: Class, trueAnomaly: Double) -> Double;

    /// The true anomaly at the given orbital radius.
    /// 
    /// <param name="radius">The orbital radius in meters.</param>
    fn Orbit_TrueAnomalyAtRadius(this: Class, radius: Double) -> Double;

    /// The true anomaly at the given time.
    /// 
    /// <param name="ut">The universal time in seconds.</param>
    fn Orbit_TrueAnomalyAtUT(this: Class, ut: Double) -> Double;

    /// The universal time, in seconds, corresponding to the given true anomaly.
    /// 
    /// <param name="trueAnomaly">True anomaly.</param>
    fn Orbit_UTAtTrueAnomaly(this: Class, trueAnomaly: Double) -> Double;

    /// The eccentric anomaly at the given universal time.
    /// 
    /// <param name="ut">The universal time, in seconds.</param>
    fn Orbit_EccentricAnomalyAtUT(this: Class, ut: Double) -> Double;

    /// The orbital speed at the given time, in meters per second.
    /// 
    /// <param name="time">Time from now, in seconds.</param>
    fn Orbit_OrbitalSpeedAt(this: Class, time: Double) -> Double;

    /// The orbital radius at the given time, in meters.
    /// 
    /// <param name="ut">The universal time to measure the radius at.</param>
    fn Orbit_RadiusAt(this: Class, ut: Double) -> Double;

    /// The position at a given time, in the specified reference frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="ut">The universal time to measure the position at.</param>
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn Orbit_PositionAt(this: Class, ut: Double, referenceFrame: Class) -> Tuple;

    /// Estimates and returns the time at closest approach to a target orbit.
    /// 
    /// # Returns
    /// 
    /// The universal time at closest approach, in seconds.
    /// 
    /// <param name="target">Target orbit.</param>
    fn Orbit_TimeOfClosestApproach(this: Class, target: Class) -> Double;

    /// Estimates and returns the distance at closest approach to a target
    /// orbit, in meters.
    /// 
    /// <param name="target">Target orbit.</param>
    fn Orbit_DistanceAtClosestApproach(this: Class, target: Class) -> Double;

    /// Returns the times at closest approach and corresponding distances, to a
    /// target orbit.
    /// 
    /// # Returns
    /// 
    /// 
    /// A list of two lists.
    /// The first is a list of times at closest approach, as universal times in
    /// seconds.
    /// The second is a list of corresponding distances at closest approach, in
    /// meters.
    /// 
    /// 
    /// <param name="target">Target orbit.</param>
    /// <param name="orbits">The number of future orbits to search.</param>
    fn Orbit_ListClosestApproaches(this: Class, target: Class, orbits: Sint32) -> List;

    /// The true anomaly of the ascending node with the given target orbit.
    /// 
    /// <param name="target">Target orbit.</param>
    fn Orbit_TrueAnomalyAtAN(this: Class, target: Class) -> Double;

    /// The true anomaly of the descending node with the given target orbit.
    /// 
    /// <param name="target">Target orbit.</param>
    fn Orbit_TrueAnomalyAtDN(this: Class, target: Class) -> Double;

    /// Relative inclination of this orbit and the target orbit, in radians.
    /// 
    /// <param name="target">Target orbit.</param>
    fn Orbit_RelativeInclination(this: Class, target: Class) -> Double;

    /// The celestial body (e.g. planet or moon) around which the object is
    /// orbiting.
    fn Orbit_get_Body(this: Class) -> Class;

    /// Gets the apoapsis of the orbit, in meters, from the center of mass
    /// of the body being orbited.
    /// 
    /// <remarks>
    /// For the apoapsis altitude reported on the in-game map view,
    /// use SpaceCenter.Orbit.ApoapsisAltitude.
    /// </remarks>
    fn Orbit_get_Apoapsis(this: Class) -> Double;

    /// The periapsis of the orbit, in meters, from the center of mass
    /// of the body being orbited.
    /// 
    /// <remarks>
    /// For the periapsis altitude reported on the in-game map view,
    /// use SpaceCenter.Orbit.PeriapsisAltitude.
    /// </remarks>
    fn Orbit_get_Periapsis(this: Class) -> Double;

    /// The apoapsis of the orbit, in meters, above the sea level of the body
    /// being orbited.
    /// 
    /// <remarks>
    /// This is equal to SpaceCenter.Orbit.Apoapsis minus the equatorial radius
    /// of the body.
    /// </remarks>
    fn Orbit_get_ApoapsisAltitude(this: Class) -> Double;

    /// The periapsis of the orbit, in meters, above the sea level of the body
    /// being orbited.
    /// 
    /// <remarks>
    /// This is equal to SpaceCenter.Orbit.Periapsis minus the equatorial
    /// radius of the body.
    /// </remarks>
    fn Orbit_get_PeriapsisAltitude(this: Class) -> Double;

    /// The semi-major axis of the orbit, in meters.
    fn Orbit_get_SemiMajorAxis(this: Class) -> Double;

    /// The semi-minor axis of the orbit, in meters.
    fn Orbit_get_SemiMinorAxis(this: Class) -> Double;

    /// The current radius of the orbit, in meters. This is the distance
    /// between the center
    /// of mass of the object in orbit, and the center of mass of the body
    /// around which it
    /// is orbiting.
    /// 
    /// <remarks>
    /// This value will change over time if the orbit is elliptical.
    /// </remarks>
    fn Orbit_get_Radius(this: Class) -> Double;

    /// The current orbital speed of the object in meters per second.
    /// 
    /// <remarks>
    /// This value will change over time if the orbit is elliptical.
    /// </remarks>
    fn Orbit_get_Speed(this: Class) -> Double;

    /// The orbital period, in seconds.
    fn Orbit_get_Period(this: Class) -> Double;

    /// The time until the object reaches apoapsis, in seconds.
    fn Orbit_get_TimeToApoapsis(this: Class) -> Double;

    /// The time until the object reaches periapsis, in seconds.
    fn Orbit_get_TimeToPeriapsis(this: Class) -> Double;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Orbital_eccentricity">eccentricity</a>
    /// 
    /// of the orbit.
    fn Orbit_get_Eccentricity(this: Class) -> Double;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Orbital_inclination">inclination</a>
    /// of the orbit,
    /// in radians.
    fn Orbit_get_Inclination(this: Class) -> Double;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node">longitude of
    /// 
    /// the ascending node</a>, in radians.
    fn Orbit_get_LongitudeOfAscendingNode(this: Class) -> Double;

    /// The <a
    /// href="https://en.wikipedia.org/wiki/Argument_of_periapsis">argument of
    /// periapsis</a>, in radians.
    fn Orbit_get_ArgumentOfPeriapsis(this: Class) -> Double;

    /// The <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean anomaly
    /// at epoch</a>.
    fn Orbit_get_MeanAnomalyAtEpoch(this: Class) -> Double;

    /// The time since the epoch (the point at which the
    /// <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean anomaly at
    /// epoch</a>
    /// was measured, in seconds.
    fn Orbit_get_Epoch(this: Class) -> Double;

    /// The <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean
    /// anomaly</a>.
    fn Orbit_get_MeanAnomaly(this: Class) -> Double;

    /// The <a href="https://en.wikipedia.org/wiki/Eccentric_anomaly">eccentric
    /// anomaly</a>.
    fn Orbit_get_EccentricAnomaly(this: Class) -> Double;

    /// The <a href="https://en.wikipedia.org/wiki/True_anomaly">true
    /// anomaly</a>.
    fn Orbit_get_TrueAnomaly(this: Class) -> Double;

    /// If the object is going to change sphere of influence in the future,
    /// returns the new
    /// orbit after the change. Otherwise returns <c>null</c>.
    fn Orbit_get_NextOrbit(this: Class) -> Option<Class>;

    /// The time until the object changes sphere of influence, in seconds.
    /// Returns <c>NaN</c>
    /// if the object is not going to change sphere of influence.
    fn Orbit_get_TimeToSOIChange(this: Class) -> Double;

    /// The current orbital speed in meters per second.
    fn Orbit_get_OrbitalSpeed(this: Class) -> Double;

    /// Transmit data.
    fn Antenna_Transmit(this: Class);

    /// Cancel current transmission of data.
    fn Antenna_Cancel(this: Class);

    /// The part object for this antenna.
    fn Antenna_get_Part(this: Class) -> Class;

    /// The current state of the antenna.
    fn Antenna_get_State(this: Class) -> Enumeration;

    /// Whether the antenna is deployable.
    fn Antenna_get_Deployable(this: Class) -> Bool;

    /// Whether the antenna is deployed.
    /// 
    /// <remarks>
    /// Fixed antennas are always deployed.
    /// Returns an error if you try to deploy a fixed antenna.
    /// </remarks>
    fn Antenna_get_Deployed(this: Class) -> Bool;

    /// Whether the antenna is deployed.
    /// 
    /// <remarks>
    /// Fixed antennas are always deployed.
    /// Returns an error if you try to deploy a fixed antenna.
    /// </remarks>
    fn Antenna_set_Deployed(this: Class, value: Bool);

    /// Whether data can be transmitted by this antenna.
    fn Antenna_get_CanTransmit(this: Class) -> Bool;

    /// Whether partial data transmission is permitted.
    fn Antenna_get_AllowPartial(this: Class) -> Bool;

    /// Whether partial data transmission is permitted.
    fn Antenna_set_AllowPartial(this: Class, value: Bool);

    /// The power of the antenna.
    fn Antenna_get_Power(this: Class) -> Double;

    /// Whether the antenna can be combined with other antennae on the vessel
    /// to boost the power.
    fn Antenna_get_Combinable(this: Class) -> Bool;

    /// Exponent used to calculate the combined power of multiple antennae on a
    /// vessel.
    fn Antenna_get_CombinableExponent(this: Class) -> Double;

    /// Interval between sending packets in seconds.
    fn Antenna_get_PacketInterval(this: Class) -> Float;

    /// Amount of data sent per packet in Mits.
    fn Antenna_get_PacketSize(this: Class) -> Float;

    /// Units of electric charge consumed per packet sent.
    fn Antenna_get_PacketResourceCost(this: Class) -> Double;

    /// The part object for this cargo bay.
    fn CargoBay_get_Part(this: Class) -> Class;

    /// The state of the cargo bay.
    fn CargoBay_get_State(this: Class) -> Enumeration;

    /// Whether the cargo bay is open.
    fn CargoBay_get_Open(this: Class) -> Bool;

    /// Whether the cargo bay is open.
    fn CargoBay_set_Open(this: Class, value: Bool);

    /// The part object for this control surface.
    fn ControlSurface_get_Part(this: Class) -> Class;

    /// Whether the control surface has pitch control enabled.
    fn ControlSurface_get_PitchEnabled(this: Class) -> Bool;

    /// Whether the control surface has pitch control enabled.
    fn ControlSurface_set_PitchEnabled(this: Class, value: Bool);

    /// Whether the control surface has yaw control enabled.
    fn ControlSurface_get_YawEnabled(this: Class) -> Bool;

    /// Whether the control surface has yaw control enabled.
    fn ControlSurface_set_YawEnabled(this: Class, value: Bool);

    /// Whether the control surface has roll control enabled.
    fn ControlSurface_get_RollEnabled(this: Class) -> Bool;

    /// Whether the control surface has roll control enabled.
    fn ControlSurface_set_RollEnabled(this: Class, value: Bool);

    /// The authority limiter for the control surface, which controls how far
    /// the
    /// control surface will move.
    fn ControlSurface_get_AuthorityLimiter(this: Class) -> Float;

    /// The authority limiter for the control surface, which controls how far
    /// the
    /// control surface will move.
    fn ControlSurface_set_AuthorityLimiter(this: Class, value: Float);

    /// Whether the control surface movement is inverted.
    fn ControlSurface_get_Inverted(this: Class) -> Bool;

    /// Whether the control surface movement is inverted.
    fn ControlSurface_set_Inverted(this: Class, value: Bool);

    /// Whether the control surface has been fully deployed.
    fn ControlSurface_get_Deployed(this: Class) -> Bool;

    /// Whether the control surface has been fully deployed.
    fn ControlSurface_set_Deployed(this: Class, value: Bool);

    /// Surface area of the control surface in <math>m^2</math>.
    fn ControlSurface_get_SurfaceArea(this: Class) -> Float;

    /// The available torque, in Newton meters, that can be produced by this
    /// control surface,
    /// in the positive and negative pitch, roll and yaw axes of the vessel.
    /// These axes
    /// correspond to the coordinate axes of the
    /// SpaceCenter.Vessel.ReferenceFrame.
    fn ControlSurface_get_AvailableTorque(this: Class) -> Tuple;

    /// Fires the decoupler. Returns the new vessel created when the decoupler
    /// fires.
    /// Throws an exception if the decoupler has already fired.
    /// 
    /// <remarks>
    /// When called, the active vessel may change. It is therefore possible
    /// that,
    /// after calling this function, the object(s) returned by previous call(s)
    /// to
    /// SpaceCenter.ActiveVessel no longer refer to the active vessel.
    /// </remarks>
    fn Decoupler_Decouple(this: Class) -> Class;

    /// The part object for this decoupler.
    fn Decoupler_get_Part(this: Class) -> Class;

    /// Whether the decoupler has fired.
    fn Decoupler_get_Decoupled(this: Class) -> Bool;

    /// Whether the decoupler is enabled in the staging sequence.
    fn Decoupler_get_Staged(this: Class) -> Bool;

    /// The impulse that the decoupler imparts when it is fired, in Newton
    /// seconds.
    fn Decoupler_get_Impulse(this: Class) -> Float;

    /// Undocks the docking port and returns the new <see
    /// cref="T:SpaceCenter.Vessel that is created.
    /// This method can be called for either docking port in a docked pair.
    /// Throws an exception if the docking port is not docked to anything.
    /// 
    /// <remarks>
    /// When called, the active vessel may change. It is therefore possible
    /// that,
    /// after calling this function, the object(s) returned by previous call(s)
    /// to
    /// SpaceCenter.ActiveVessel no longer refer to the active vessel.
    /// </remarks>
    fn DockingPort_Undock(this: Class) -> Class;

    /// The position of the docking port, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn DockingPort_Position(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction that docking port points in, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn DockingPort_Direction(this: Class, referenceFrame: Class) -> Tuple;

    /// The rotation of the docking port, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// rotation is in.</param>
    fn DockingPort_Rotation(this: Class, referenceFrame: Class) -> Tuple;

    /// The part object for this docking port.
    fn DockingPort_get_Part(this: Class) -> Class;

    /// The current state of the docking port.
    fn DockingPort_get_State(this: Class) -> Enumeration;

    /// The part that this docking port is docked to. Returns <c>null</c> if
    /// this
    /// docking port is not docked to anything.
    fn DockingPort_get_DockedPart(this: Class) -> Option<Class>;

    /// The distance a docking port must move away when it undocks before it
    /// becomes ready to dock with another port, in meters.
    fn DockingPort_get_ReengageDistance(this: Class) -> Float;

    /// Whether the docking port has a shield.
    fn DockingPort_get_HasShield(this: Class) -> Bool;

    /// The state of the docking ports shield, if it has one.
    /// 
    /// Returns <c>true</c> if the docking port has a shield, and the shield is
    /// closed. Otherwise returns <c>false</c>. When set to <c>true</c>, the
    /// shield is
    /// closed, and when set to <c>false</c> the shield is opened. If the
    /// docking
    /// port does not have a shield, setting this attribute has no effect.
    fn DockingPort_get_Shielded(this: Class) -> Bool;

    /// The state of the docking ports shield, if it has one.
    /// 
    /// Returns <c>true</c> if the docking port has a shield, and the shield is
    /// closed. Otherwise returns <c>false</c>. When set to <c>true</c>, the
    /// shield is
    /// closed, and when set to <c>false</c> the shield is opened. If the
    /// docking
    /// port does not have a shield, setting this attribute has no effect.
    fn DockingPort_set_Shielded(this: Class, value: Bool);

    /// The reference frame that is fixed relative to this docking port, and
    /// oriented with the port.
    /// <list type="bullet"><item><description>The origin is at the position of
    /// the docking port.
    /// </description></item><item><description>The axes rotate with the
    /// docking port.</description></item><item><description>The x-axis points
    /// out to the right side of the docking port.
    /// </description></item><item><description>The y-axis points in the
    /// direction the docking port is facing.
    /// </description></item><item><description>The z-axis points out of the
    /// bottom off the docking port.
    /// </description></item></list>
    /// <remarks>
    /// This reference frame is not necessarily equivalent to the reference
    /// frame
    /// for the part, returned by SpaceCenter.Part.ReferenceFrame.
    /// </remarks>
    fn DockingPort_get_ReferenceFrame(this: Class) -> Class;

    /// Toggle the current engine mode.
    fn Engine_ToggleMode(this: Class);

    /// The part object for this engine.
    fn Engine_get_Part(this: Class) -> Class;

    /// Whether the engine is active. Setting this attribute may have no effect,
    /// depending on SpaceCenter.Engine.CanShutdown and
    /// SpaceCenter.Engine.CanRestart.
    fn Engine_get_Active(this: Class) -> Bool;

    /// Whether the engine is active. Setting this attribute may have no effect,
    /// depending on SpaceCenter.Engine.CanShutdown and
    /// SpaceCenter.Engine.CanRestart.
    fn Engine_set_Active(this: Class, value: Bool);

    /// The current amount of thrust being produced by the engine, in Newtons.
    fn Engine_get_Thrust(this: Class) -> Float;

    /// The amount of thrust, in Newtons, that would be produced by the engine
    /// when activated and with its throttle set to 100%.
    /// Returns zero if the engine does not have any fuel.
    /// Takes the engine's current SpaceCenter.Engine.ThrustLimit and
    /// atmospheric conditions
    /// into account.
    fn Engine_get_AvailableThrust(this: Class) -> Float;

    /// The amount of thrust, in Newtons, that would be produced by the engine
    /// when activated and fueled, with its throttle and throttle limiter set
    /// to 100%.
    fn Engine_get_MaxThrust(this: Class) -> Float;

    /// The maximum amount of thrust that can be produced by the engine in a
    /// vacuum, in Newtons. This is the amount of thrust produced by the engine
    /// when activated, SpaceCenter.Engine.ThrustLimit is set to 100%, the main
    /// vessel's throttle is set to 100% and the engine is in a vacuum.
    fn Engine_get_MaxVacuumThrust(this: Class) -> Float;

    /// The thrust limiter of the engine. A value between 0 and 1. Setting this
    /// attribute may have no effect, for example the thrust limit for a solid
    /// rocket booster cannot be changed in flight.
    fn Engine_get_ThrustLimit(this: Class) -> Float;

    /// The thrust limiter of the engine. A value between 0 and 1. Setting this
    /// attribute may have no effect, for example the thrust limit for a solid
    /// rocket booster cannot be changed in flight.
    fn Engine_set_ThrustLimit(this: Class, value: Float);

    /// The components of the engine that generate thrust.
    /// 
    /// <remarks>
    /// For example, this corresponds to the rocket nozzel on a solid rocket
    /// booster,
    /// or the individual nozzels on a RAPIER engine.
    /// The overall thrust produced by the engine, as reported by
    /// SpaceCenter.Engine.AvailableThrust,
    /// SpaceCenter.Engine.MaxThrust and others, is the sum of the thrust
    /// generated by each thruster.
    /// </remarks>
    fn Engine_get_Thrusters(this: Class) -> List;

    /// The current specific impulse of the engine, in seconds. Returns zero
    /// if the engine is not active.
    fn Engine_get_SpecificImpulse(this: Class) -> Float;

    /// The vacuum specific impulse of the engine, in seconds.
    fn Engine_get_VacuumSpecificImpulse(this: Class) -> Float;

    /// The specific impulse of the engine at sea level on Kerbin, in seconds.
    fn Engine_get_KerbinSeaLevelSpecificImpulse(this: Class) -> Float;

    /// The names of the propellants that the engine consumes.
    fn Engine_get_PropellantNames(this: Class) -> List;

    /// The propellants that the engine consumes.
    fn Engine_get_Propellants(this: Class) -> List;

    /// The ratio of resources that the engine consumes. A dictionary mapping
    /// resource names
    /// to the ratio at which they are consumed by the engine.
    /// 
    /// <remarks>
    /// For example, if the ratios are 0.6 for LiquidFuel and 0.4 for Oxidizer,
    /// then for every
    /// 0.6 units of LiquidFuel that the engine burns, it will burn 0.4 units
    /// of Oxidizer.
    /// </remarks>
    fn Engine_get_PropellantRatios(this: Class) -> Dictionary;

    /// Whether the engine has any fuel available.
    /// 
    /// <remarks>
    /// The engine must be activated for this property to update correctly.
    /// </remarks>
    fn Engine_get_HasFuel(this: Class) -> Bool;

    /// The current throttle setting for the engine. A value between 0 and 1.
    /// This is not necessarily the same as the vessel's main throttle
    /// setting, as some engines take time to adjust their throttle
    /// (such as jet engines).
    fn Engine_get_Throttle(this: Class) -> Float;

    /// Whether the SpaceCenter.Control.Throttle affects the engine. For
    /// example,
    /// this is <c>true</c> for liquid fueled rockets, and <c>false</c> for
    /// solid rocket
    /// boosters.
    fn Engine_get_ThrottleLocked(this: Class) -> Bool;

    /// Whether the engine can be restarted once shutdown. If the engine cannot
    /// be shutdown,
    /// returns <c>false</c>. For example, this is <c>true</c> for liquid
    /// fueled rockets
    /// and <c>false</c> for solid rocket boosters.
    fn Engine_get_CanRestart(this: Class) -> Bool;

    /// Whether the engine can be shutdown once activated. For example, this is
    /// <c>true</c> for liquid fueled rockets and <c>false</c> for solid rocket
    /// boosters.
    fn Engine_get_CanShutdown(this: Class) -> Bool;

    /// Whether the engine has multiple modes of operation.
    fn Engine_get_HasModes(this: Class) -> Bool;

    /// The name of the current engine mode.
    fn Engine_get_Mode(this: Class) -> String;

    /// The name of the current engine mode.
    fn Engine_set_Mode(this: Class, value: String);

    /// The available modes for the engine.
    /// A dictionary mapping mode names to <see cref="T:SpaceCenter.Engine
    /// objects.
    fn Engine_get_Modes(this: Class) -> Dictionary;

    /// Whether the engine will automatically switch modes.
    fn Engine_get_AutoModeSwitch(this: Class) -> Bool;

    /// Whether the engine will automatically switch modes.
    fn Engine_set_AutoModeSwitch(this: Class, value: Bool);

    /// Whether the engine is gimballed.
    fn Engine_get_Gimballed(this: Class) -> Bool;

    /// The range over which the gimbal can move, in degrees.
    /// Returns 0 if the engine is not gimballed.
    fn Engine_get_GimbalRange(this: Class) -> Float;

    /// Whether the engines gimbal is locked in place. Setting this attribute
    /// has
    /// no effect if the engine is not gimballed.
    fn Engine_get_GimbalLocked(this: Class) -> Bool;

    /// Whether the engines gimbal is locked in place. Setting this attribute
    /// has
    /// no effect if the engine is not gimballed.
    fn Engine_set_GimbalLocked(this: Class, value: Bool);

    /// The gimbal limiter of the engine. A value between 0 and 1.
    /// Returns 0 if the gimbal is locked.
    fn Engine_get_GimbalLimit(this: Class) -> Float;

    /// The gimbal limiter of the engine. A value between 0 and 1.
    /// Returns 0 if the gimbal is locked.
    fn Engine_set_GimbalLimit(this: Class, value: Float);

    /// The available torque, in Newton meters, that can be produced by this
    /// engine,
    /// in the positive and negative pitch, roll and yaw axes of the vessel.
    /// These axes
    /// correspond to the coordinate axes of the
    /// SpaceCenter.Vessel.ReferenceFrame.
    /// Returns zero if the engine is inactive, or not gimballed.
    fn Engine_get_AvailableTorque(this: Class) -> Tuple;

    /// Run the experiment.
    fn Experiment_Run(this: Class);

    /// Transmit all experimental data contained by this part.
    fn Experiment_Transmit(this: Class);

    /// Dump the experimental data contained by the experiment.
    fn Experiment_Dump(this: Class);

    /// Reset the experiment.
    fn Experiment_Reset(this: Class);

    /// The part object for this experiment.
    fn Experiment_get_Part(this: Class) -> Class;

    /// Whether the experiment is inoperable.
    fn Experiment_get_Inoperable(this: Class) -> Bool;

    /// Whether the experiment has been deployed.
    fn Experiment_get_Deployed(this: Class) -> Bool;

    /// Whether the experiment can be re-run.
    fn Experiment_get_Rerunnable(this: Class) -> Bool;

    /// Whether the experiment contains data.
    fn Experiment_get_HasData(this: Class) -> Bool;

    /// The data contained in this experiment.
    fn Experiment_get_Data(this: Class) -> List;

    /// Determines if the experiment is available given the current conditions.
    fn Experiment_get_Available(this: Class) -> Bool;

    /// The name of the biome the experiment is currently in.
    fn Experiment_get_Biome(this: Class) -> String;

    /// Containing information on the corresponding specific science result for
    /// the current
    /// conditions. Returns <c>null</c> if the experiment is unavailable.
    fn Experiment_get_ScienceSubject(this: Class) -> Class;

    /// Jettison the fairing. Has no effect if it has already been jettisoned.
    fn Fairing_Jettison(this: Class);

    /// The part object for this fairing.
    fn Fairing_get_Part(this: Class) -> Class;

    /// Whether the fairing has been jettisoned.
    fn Fairing_get_Jettisoned(this: Class) -> Bool;

    /// Remove the force.
    fn Force_Remove(this: Class);

    /// The part that this force is applied to.
    fn Force_get_Part(this: Class) -> Class;

    /// The force vector, in Newtons.
    /// 
    /// # Returns
    /// 
    /// A vector pointing in the direction that the force acts,
    /// with its magnitude equal to the strength of the force in Newtons.
    fn Force_get_ForceVector(this: Class) -> Tuple;

    /// The force vector, in Newtons.
    /// 
    /// # Returns
    /// 
    /// A vector pointing in the direction that the force acts,
    /// with its magnitude equal to the strength of the force in Newtons.
    fn Force_set_ForceVector(this: Class, value: Tuple);

    /// The position at which the force acts, in reference frame <see
    /// cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    fn Force_get_Position(this: Class) -> Tuple;

    /// The position at which the force acts, in reference frame <see
    /// cref="T:SpaceCenter.ReferenceFrame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    fn Force_set_Position(this: Class, value: Tuple);

    /// The reference frame of the force vector and position.
    fn Force_get_ReferenceFrame(this: Class) -> Class;

    /// The reference frame of the force vector and position.
    fn Force_set_ReferenceFrame(this: Class, value: Class);

    /// The part object for this intake.
    fn Intake_get_Part(this: Class) -> Class;

    /// Whether the intake is open.
    fn Intake_get_Open(this: Class) -> Bool;

    /// Whether the intake is open.
    fn Intake_set_Open(this: Class, value: Bool);

    /// Speed of the flow into the intake, in <math>m/s</math>.
    fn Intake_get_Speed(this: Class) -> Float;

    /// The rate of flow into the intake, in units of resource per second.
    fn Intake_get_Flow(this: Class) -> Float;

    /// The area of the intake's opening, in square meters.
    fn Intake_get_Area(this: Class) -> Float;

    /// Releases the docking clamp. Has no effect if the clamp has already been
    /// released.
    fn LaunchClamp_Release(this: Class);

    /// The part object for this launch clamp.
    fn LaunchClamp_get_Part(this: Class) -> Class;

    /// The part object for this landing leg.
    fn Leg_get_Part(this: Class) -> Class;

    /// The current state of the landing leg.
    fn Leg_get_State(this: Class) -> Enumeration;

    /// Whether the leg is deployable.
    fn Leg_get_Deployable(this: Class) -> Bool;

    /// Whether the landing leg is deployed.
    /// 
    /// <remarks>
    /// Fixed landing legs are always deployed.
    /// Returns an error if you try to deploy fixed landing gear.
    /// </remarks>
    fn Leg_get_Deployed(this: Class) -> Bool;

    /// Whether the landing leg is deployed.
    /// 
    /// <remarks>
    /// Fixed landing legs are always deployed.
    /// Returns an error if you try to deploy fixed landing gear.
    /// </remarks>
    fn Leg_set_Deployed(this: Class, value: Bool);

    /// Returns whether the leg is touching the ground.
    fn Leg_get_IsGrounded(this: Class) -> Bool;

    /// The part object for this light.
    fn Light_get_Part(this: Class) -> Class;

    /// Whether the light is switched on.
    fn Light_get_Active(this: Class) -> Bool;

    /// Whether the light is switched on.
    fn Light_set_Active(this: Class, value: Bool);

    /// The color of the light, as an RGB triple.
    fn Light_get_Color(this: Class) -> Tuple;

    /// The color of the light, as an RGB triple.
    fn Light_set_Color(this: Class, value: Tuple);

    /// The current power usage, in units of charge per second.
    fn Light_get_PowerUsage(this: Class) -> Float;

    /// Returns <c>true</c> if the module has a field with the given name.
    /// 
    /// <param name="name">Name of the field.</param>
    fn Module_HasField(this: Class, name: String) -> Bool;

    /// Returns the value of a field.
    /// 
    /// <param name="name">Name of the field.</param>
    fn Module_GetField(this: Class, name: String) -> String;

    /// Set the value of a field to the given integer number.
    /// 
    /// <param name="name">Name of the field.</param>
    /// <param name="value">Value to set.</param>
    fn Module_SetFieldInt(this: Class, name: String, value: Sint32);

    /// Set the value of a field to the given floating point number.
    /// 
    /// <param name="name">Name of the field.</param>
    /// <param name="value">Value to set.</param>
    fn Module_SetFieldFloat(this: Class, name: String, value: Float);

    /// Set the value of a field to the given string.
    /// 
    /// <param name="name">Name of the field.</param>
    /// <param name="value">Value to set.</param>
    fn Module_SetFieldString(this: Class, name: String, value: String);

    /// Set the value of a field to its original value.
    /// 
    /// <param name="name">Name of the field.</param>
    fn Module_ResetField(this: Class, name: String);

    /// <c>true</c> if the module has an event with the given name.
    /// 
    /// <param name="name"></param>
    fn Module_HasEvent(this: Class, name: String) -> Bool;

    /// Trigger the named event. Equivalent to clicking the button in the
    /// right-click menu
    /// of the part.
    /// 
    /// <param name="name"></param>
    fn Module_TriggerEvent(this: Class, name: String);

    /// <c>true</c> if the part has an action with the given name.
    /// 
    /// <param name="name"></param>
    fn Module_HasAction(this: Class, name: String) -> Bool;

    /// Set the value of an action with the given name.
    /// 
    /// <param name="name"></param>
    /// <param name="value"></param>
    fn Module_SetAction(this: Class, name: String, value: Bool);

    /// Name of the PartModule. For example, "ModuleEngines".
    fn Module_get_Name(this: Class) -> String;

    /// The part that contains this module.
    fn Module_get_Part(this: Class) -> Class;

    /// The modules field names and their associated values, as a dictionary.
    /// These are the values visible in the right-click menu of the part.
    fn Module_get_Fields(this: Class) -> Dictionary;

    /// A list of the names of all of the modules events. Events are the
    /// clickable buttons
    /// visible in the right-click menu of the part.
    fn Module_get_Events(this: Class) -> List;

    /// A list of all the names of the modules actions. These are the parts
    /// actions that can
    /// be assigned to action groups in the in-game editor.
    fn Module_get_Actions(this: Class) -> List;

    /// Deploys the parachute. This has no effect if the parachute has already
    /// been deployed.
    fn Parachute_Deploy(this: Class);

    /// Deploys the parachute. This has no effect if the parachute has already
    /// been armed or deployed. Only applicable to RealChutes parachutes.
    fn Parachute_Arm(this: Class);

    /// The part object for this parachute.
    fn Parachute_get_Part(this: Class) -> Class;

    /// Whether the parachute has been deployed.
    fn Parachute_get_Deployed(this: Class) -> Bool;

    /// Whether the parachute has been armed or deployed. Only applicable to
    /// RealChutes parachutes.
    fn Parachute_get_Armed(this: Class) -> Bool;

    /// The current state of the parachute.
    fn Parachute_get_State(this: Class) -> Enumeration;

    /// The altitude at which the parachute will full deploy, in meters.
    /// Only applicable to stock parachutes.
    fn Parachute_get_DeployAltitude(this: Class) -> Float;

    /// The altitude at which the parachute will full deploy, in meters.
    /// Only applicable to stock parachutes.
    fn Parachute_set_DeployAltitude(this: Class, value: Float);

    /// The minimum pressure at which the parachute will semi-deploy, in
    /// atmospheres.
    /// Only applicable to stock parachutes.
    fn Parachute_get_DeployMinPressure(this: Class) -> Float;

    /// The minimum pressure at which the parachute will semi-deploy, in
    /// atmospheres.
    /// Only applicable to stock parachutes.
    fn Parachute_set_DeployMinPressure(this: Class, value: Float);

    /// The position of the part in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    /// <remarks>
    /// This is a fixed position in the part, defined by the parts model.
    /// It s not necessarily the same as the parts center of mass.
    /// Use SpaceCenter.Part.CenterOfMass to get the parts center of mass.
    /// </remarks>
    fn Part_Position(this: Class, referenceFrame: Class) -> Tuple;

    /// The position of the parts center of mass in the given reference frame.
    /// If the part is physicsless, this is equivalent to
    /// SpaceCenter.Part.Position.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn Part_CenterOfMass(this: Class, referenceFrame: Class) -> Tuple;

    /// The axis-aligned bounding box of the part in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The positions of the minimum and maximum vertices of the box,
    /// as position vectors.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vectors are in.</param>
    /// <remarks>
    /// This is computed from the collision mesh of the part.
    /// If the part is not collidable, the box has zero volume and is centered
    /// on
    /// the SpaceCenter.Part.Position of the part.
    /// </remarks>
    fn Part_BoundingBox(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction the part points in, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Part_Direction(this: Class, referenceFrame: Class) -> Tuple;

    /// The linear velocity of the part in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The velocity as a vector. The vector points in the direction of travel,
    /// and its magnitude is the speed of the body in meters per second.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// velocity vector is in.</param>
    fn Part_Velocity(this: Class, referenceFrame: Class) -> Tuple;

    /// The rotation of the part, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// rotation is in.</param>
    fn Part_Rotation(this: Class, referenceFrame: Class) -> Tuple;

    /// Exert a constant force on the part, acting at the given position.
    /// 
    /// # Returns
    /// 
    /// An object that can be used to remove or modify the force.
    /// 
    /// <param name="force">A vector pointing in the direction that the force
    /// acts,
    /// with its magnitude equal to the strength of the force in
    /// Newtons.</param>
    /// <param name="position">The position at which the force acts, as a
    /// vector.</param>
    /// <param name="referenceFrame">The reference frame that the
    /// force and position are in.</param>
    fn Part_AddForce(this: Class, force: Tuple, position: Tuple, referenceFrame: Class) -> Class;

    /// Exert an instantaneous force on the part, acting at the given position.
    /// 
    /// <param name="force">A vector pointing in the direction that the force
    /// acts,
    /// with its magnitude equal to the strength of the force in
    /// Newtons.</param>
    /// <param name="position">The position at which the force acts, as a
    /// vector.</param>
    /// <param name="referenceFrame">The reference frame that the
    /// force and position are in.</param>
    /// <remarks>The force is applied instantaneously in a single physics
    /// update.</remarks>
    fn Part_InstantaneousForce(this: Class, force: Tuple, position: Tuple, referenceFrame: Class);

    /// Internal name of the part, as used in
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/CFG_File_Documentation">part cfg files</a>.
    /// 
    /// For example "Mark1-2Pod".
    fn Part_get_Name(this: Class) -> String;

    /// Title of the part, as shown when the part is right clicked in-game. For
    /// example "Mk1-2 Command Pod".
    fn Part_get_Title(this: Class) -> String;

    /// The name tag for the part. Can be set to a custom string using the
    /// in-game user interface.
    /// 
    /// <remarks>
    /// This string is shared with
    /// <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/61827-/">kOS</a>
    /// 
    /// if it is installed.
    /// </remarks>
    fn Part_get_Tag(this: Class) -> String;

    /// The name tag for the part. Can be set to a custom string using the
    /// in-game user interface.
    /// 
    /// <remarks>
    /// This string is shared with
    /// <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/61827-/">kOS</a>
    /// 
    /// if it is installed.
    /// </remarks>
    fn Part_set_Tag(this: Class, value: String);

    /// Whether the part is highlighted.
    fn Part_get_Highlighted(this: Class) -> Bool;

    /// Whether the part is highlighted.
    fn Part_set_Highlighted(this: Class, value: Bool);

    /// The color used to highlight the part, as an RGB triple.
    fn Part_get_HighlightColor(this: Class) -> Tuple;

    /// The color used to highlight the part, as an RGB triple.
    fn Part_set_HighlightColor(this: Class, value: Tuple);

    /// The cost of the part, in units of funds.
    fn Part_get_Cost(this: Class) -> Double;

    /// The vessel that contains this part.
    fn Part_get_Vessel(this: Class) -> Class;

    /// The parts parent. Returns <c>null</c> if the part does not have a
    /// parent.
    /// This, in combination with SpaceCenter.Part.Children, can be used to
    /// traverse the vessels
    /// parts tree.
    fn Part_get_Parent(this: Class) -> Option<Class>;

    /// The parts children. Returns an empty list if the part has no children.
    /// This, in combination with SpaceCenter.Part.Parent, can be used to
    /// traverse the vessels
    /// parts tree.
    fn Part_get_Children(this: Class) -> List;

    /// Whether the part is axially attached to its parent, i.e. on the top
    /// or bottom of its parent. If the part has no parent, returns
    /// <c>false</c>.
    fn Part_get_AxiallyAttached(this: Class) -> Bool;

    /// Whether the part is radially attached to its parent, i.e. on the side
    /// of its parent.
    /// If the part has no parent, returns <c>false</c>.
    fn Part_get_RadiallyAttached(this: Class) -> Bool;

    /// The stage in which this part will be activated. Returns -1 if the part
    /// is not
    /// activated by staging.
    fn Part_get_Stage(this: Class) -> Sint32;

    /// The stage in which this part will be decoupled. Returns -1 if the part
    /// is never
    /// decoupled from the vessel.
    fn Part_get_DecoupleStage(this: Class) -> Sint32;

    /// Whether the part is
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Massless_part">massless</a>.
    /// 
    fn Part_get_Massless(this: Class) -> Bool;

    /// The current mass of the part, including resources it contains, in
    /// kilograms.
    /// Returns zero if the part is massless.
    fn Part_get_Mass(this: Class) -> Double;

    /// The mass of the part, not including any resources it contains, in
    /// kilograms.
    /// Returns zero if the part is massless.
    fn Part_get_DryMass(this: Class) -> Double;

    /// Whether the part is shielded from the exterior of the vessel, for
    /// example by a fairing.
    fn Part_get_Shielded(this: Class) -> Bool;

    /// The dynamic pressure acting on the part, in Pascals.
    fn Part_get_DynamicPressure(this: Class) -> Float;

    /// The impact tolerance of the part, in meters per second.
    fn Part_get_ImpactTolerance(this: Class) -> Double;

    /// Temperature of the part, in Kelvin.
    fn Part_get_Temperature(this: Class) -> Double;

    /// Temperature of the skin of the part, in Kelvin.
    fn Part_get_SkinTemperature(this: Class) -> Double;

    /// Maximum temperature that the part can survive, in Kelvin.
    fn Part_get_MaxTemperature(this: Class) -> Double;

    /// Maximum temperature that the skin of the part can survive, in Kelvin.
    fn Part_get_MaxSkinTemperature(this: Class) -> Double;

    /// A measure of how much energy it takes to increase the internal
    /// temperature of the part,
    /// in Joules per Kelvin.
    fn Part_get_ThermalMass(this: Class) -> Float;

    /// A measure of how much energy it takes to increase the skin temperature
    /// of the part,
    /// in Joules per Kelvin.
    fn Part_get_ThermalSkinMass(this: Class) -> Float;

    /// A measure of how much energy it takes to increase the temperature of
    /// the resources
    /// contained in the part, in Joules per Kelvin.
    fn Part_get_ThermalResourceMass(this: Class) -> Float;

    /// The rate at which heat energy is begin generated by the part.
    /// For example, some engines generate heat by combusting fuel.
    /// Measured in energy per unit time, or power, in Watts.
    /// A positive value means the part is gaining heat energy, and negative
    /// means it is losing
    /// heat energy.
    fn Part_get_ThermalInternalFlux(this: Class) -> Float;

    /// The rate at which heat energy is conducting into or out of the part via
    /// contact with
    /// other parts. Measured in energy per unit time, or power, in Watts.
    /// A positive value means the part is gaining heat energy, and negative
    /// means it is
    /// losing heat energy.
    fn Part_get_ThermalConductionFlux(this: Class) -> Float;

    /// The rate at which heat energy is convecting into or out of the part
    /// from the
    /// surrounding atmosphere. Measured in energy per unit time, or power, in
    /// Watts.
    /// A positive value means the part is gaining heat energy, and negative
    /// means it is
    /// losing heat energy.
    fn Part_get_ThermalConvectionFlux(this: Class) -> Float;

    /// The rate at which heat energy is radiating into or out of the part from
    /// the surrounding
    /// environment. Measured in energy per unit time, or power, in Watts.
    /// A positive value means the part is gaining heat energy, and negative
    /// means it is
    /// losing heat energy.
    fn Part_get_ThermalRadiationFlux(this: Class) -> Float;

    /// The rate at which heat energy is transferring between the part's skin
    /// and its internals.
    /// Measured in energy per unit time, or power, in Watts.
    /// A positive value means the part's internals are gaining heat energy,
    /// and negative means its skin is gaining heat energy.
    fn Part_get_ThermalSkinToInternalFlux(this: Class) -> Float;

    /// A <see cref="T:SpaceCenter.Resources object for the part.
    fn Part_get_Resources(this: Class) -> Class;

    /// Whether this part is crossfeed capable.
    fn Part_get_Crossfeed(this: Class) -> Bool;

    /// Whether this part is a fuel line.
    fn Part_get_IsFuelLine(this: Class) -> Bool;

    /// The parts that are connected to this part via fuel lines, where the
    /// direction of the
    /// fuel line is into this part.
    fn Part_get_FuelLinesFrom(this: Class) -> List;

    /// The parts that are connected to this part via fuel lines, where the
    /// direction of the
    /// fuel line is out of this part.
    fn Part_get_FuelLinesTo(this: Class) -> List;

    /// The modules for this part.
    fn Part_get_Modules(this: Class) -> List;

    /// A <see cref="T:SpaceCenter.Antenna if the part is an antenna, otherwise
    /// <c>null</c>.
    fn Part_get_Antenna(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.CargoBay if the part is a cargo bay,
    /// otherwise <c>null</c>.
    fn Part_get_CargoBay(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.ControlSurface if the part is an aerodynamic
    /// control surface,
    /// otherwise <c>null</c>.
    fn Part_get_ControlSurface(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Decoupler if the part is a decoupler,
    /// otherwise <c>null</c>.
    fn Part_get_Decoupler(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.DockingPort if the part is a docking port,
    /// otherwise <c>null</c>.
    fn Part_get_DockingPort(this: Class) -> Option<Class>;

    /// An <see cref="T:SpaceCenter.Engine if the part is an engine, otherwise
    /// <c>null</c>.
    fn Part_get_Engine(this: Class) -> Option<Class>;

    /// An <see cref="T:SpaceCenter.Experiment if the part is a science
    /// experiment, otherwise <c>null</c>.
    fn Part_get_Experiment(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Fairing if the part is a fairing, otherwise
    /// <c>null</c>.
    fn Part_get_Fairing(this: Class) -> Option<Class>;

    /// An <see cref="T:SpaceCenter.Intake if the part is an intake, otherwise
    /// <c>null</c>.
    /// 
    /// <remarks>
    /// This includes any part that generates thrust. This covers many
    /// different types
    /// of engine, including liquid fuel rockets, solid rocket boosters and jet
    /// engines.
    /// For RCS thrusters see <see cref="T:SpaceCenter.RCS.
    /// </remarks>
    fn Part_get_Intake(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Leg if the part is a landing leg, otherwise
    /// <c>null</c>.
    fn Part_get_Leg(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.LaunchClamp if the part is a launch clamp,
    /// otherwise <c>null</c>.
    fn Part_get_LaunchClamp(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Light if the part is a light, otherwise
    /// <c>null</c>.
    fn Part_get_Light(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Parachute if the part is a parachute,
    /// otherwise <c>null</c>.
    fn Part_get_Parachute(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Radiator if the part is a radiator,
    /// otherwise <c>null</c>.
    fn Part_get_Radiator(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.RCS if the part is an RCS block/thruster,
    /// otherwise <c>null</c>.
    fn Part_get_RCS(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.ReactionWheel if the part is a reaction
    /// wheel, otherwise <c>null</c>.
    fn Part_get_ReactionWheel(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.ResourceConverter if the part is a resource
    /// converter,
    /// otherwise <c>null</c>.
    fn Part_get_ResourceConverter(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.ResourceHarvester if the part is a resource
    /// harvester,
    /// otherwise <c>null</c>.
    fn Part_get_ResourceHarvester(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Sensor if the part is a sensor, otherwise
    /// <c>null</c>.
    fn Part_get_Sensor(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.SolarPanel if the part is a solar panel,
    /// otherwise <c>null</c>.
    fn Part_get_SolarPanel(this: Class) -> Option<Class>;

    /// A <see cref="T:SpaceCenter.Wheel if the part is a wheel, otherwise
    /// <c>null</c>.
    fn Part_get_Wheel(this: Class) -> Option<Class>;

    /// The moment of inertia of the part in <math>kg.m^2</math> around its
    /// center of mass
    /// in the parts reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    fn Part_get_MomentOfInertia(this: Class) -> Tuple;

    /// The inertia tensor of the part in the parts reference frame
    /// (<see cref="T:SpaceCenter.ReferenceFrame).
    /// Returns the 3x3 matrix as a list of elements, in row-major order.
    fn Part_get_InertiaTensor(this: Class) -> List;

    /// The reference frame that is fixed relative to this part, and centered
    /// on a fixed
    /// position within the part, defined by the parts model.
    /// <list type="bullet"><item><description>The origin is at the position of
    /// the part, as returned by
    /// SpaceCenter.Part.Position.</description></item><item><description>The
    /// axes rotate with the part.</description></item><item><description>The
    /// x, y and z axis directions depend on the design of the part.
    /// </description></item></list>
    /// <remarks>
    /// For docking port parts, this reference frame is not necessarily
    /// equivalent to the
    /// reference frame for the docking port, returned by
    /// SpaceCenter.DockingPort.ReferenceFrame.
    /// </remarks>
    fn Part_get_ReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to this part, and centered
    /// on its
    /// center of mass.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// mass of the part, as returned by
    /// SpaceCenter.Part.CenterOfMass.</description></item><item><description>The axes rotate with the part.</description></item><item><description>The x, y and z axis directions depend on the design of the part.
    /// 
    /// </description></item></list>
    /// <remarks>
    /// For docking port parts, this reference frame is not necessarily
    /// equivalent to the
    /// reference frame for the docking port, returned by
    /// SpaceCenter.DockingPort.ReferenceFrame.
    /// </remarks>
    fn Part_get_CenterOfMassReferenceFrame(this: Class) -> Class;

    /// A list of parts whose SpaceCenter.Part.Name is <paramref name="name.
    /// 
    /// <param name="name"></param>
    fn Parts_WithName(this: Class, name: String) -> List;

    /// A list of all parts whose SpaceCenter.Part.Title is <paramref
    /// name="title.
    /// 
    /// <param name="title"></param>
    fn Parts_WithTitle(this: Class, title: String) -> List;

    /// A list of all parts whose SpaceCenter.Part.Tag is <paramref name="tag.
    /// 
    /// <param name="tag"></param>
    fn Parts_WithTag(this: Class, tag: String) -> List;

    /// A list of all parts that contain a <see cref="T:SpaceCenter.Module whose
    /// SpaceCenter.Module.Name is <paramref name="moduleName.
    /// 
    /// <param name="moduleName"></param>
    fn Parts_WithModule(this: Class, moduleName: String) -> List;

    /// A list of all parts that are activated in the given <paramref
    /// name="stage.
    /// 
    /// <param name="stage"></param>
    fn Parts_InStage(this: Class, stage: Sint32) -> List;

    /// A list of all parts that are decoupled in the given <paramref
    /// name="stage.
    /// 
    /// <param name="stage"></param>
    fn Parts_InDecoupleStage(this: Class, stage: Sint32) -> List;

    /// A list of modules (combined across all parts in the vessel) whose
    /// SpaceCenter.Module.Name is <paramref name="moduleName.
    /// 
    /// <param name="moduleName"></param>
    fn Parts_ModulesWithName(this: Class, moduleName: String) -> List;

    /// A list of all of the vessels parts.
    fn Parts_get_All(this: Class) -> List;

    /// The vessels root part.
    fn Parts_get_Root(this: Class) -> Class;

    /// The part from which the vessel is controlled.
    fn Parts_get_Controlling(this: Class) -> Class;

    /// The part from which the vessel is controlled.
    fn Parts_set_Controlling(this: Class, value: Class);

    /// A list of all antennas in the vessel.
    fn Parts_get_Antennas(this: Class) -> List;

    /// A list of all control surfaces in the vessel.
    fn Parts_get_ControlSurfaces(this: Class) -> List;

    /// A list of all cargo bays in the vessel.
    fn Parts_get_CargoBays(this: Class) -> List;

    /// A list of all decouplers in the vessel.
    fn Parts_get_Decouplers(this: Class) -> List;

    /// A list of all docking ports in the vessel.
    fn Parts_get_DockingPorts(this: Class) -> List;

    /// A list of all engines in the vessel.
    /// 
    /// <remarks>
    /// This includes any part that generates thrust. This covers many
    /// different types
    /// of engine, including liquid fuel rockets, solid rocket boosters, jet
    /// engines and
    /// RCS thrusters.
    /// </remarks>
    fn Parts_get_Engines(this: Class) -> List;

    /// A list of all science experiments in the vessel.
    fn Parts_get_Experiments(this: Class) -> List;

    /// A list of all fairings in the vessel.
    fn Parts_get_Fairings(this: Class) -> List;

    /// A list of all intakes in the vessel.
    fn Parts_get_Intakes(this: Class) -> List;

    /// A list of all landing legs attached to the vessel.
    fn Parts_get_Legs(this: Class) -> List;

    /// A list of all launch clamps attached to the vessel.
    fn Parts_get_LaunchClamps(this: Class) -> List;

    /// A list of all lights in the vessel.
    fn Parts_get_Lights(this: Class) -> List;

    /// A list of all parachutes in the vessel.
    fn Parts_get_Parachutes(this: Class) -> List;

    /// A list of all radiators in the vessel.
    fn Parts_get_Radiators(this: Class) -> List;

    /// A list of all RCS blocks/thrusters in the vessel.
    fn Parts_get_RCS(this: Class) -> List;

    /// A list of all reaction wheels in the vessel.
    fn Parts_get_ReactionWheels(this: Class) -> List;

    /// A list of all resource converters in the vessel.
    fn Parts_get_ResourceConverters(this: Class) -> List;

    /// A list of all resource harvesters in the vessel.
    fn Parts_get_ResourceHarvesters(this: Class) -> List;

    /// A list of all sensors in the vessel.
    fn Parts_get_Sensors(this: Class) -> List;

    /// A list of all solar panels in the vessel.
    fn Parts_get_SolarPanels(this: Class) -> List;

    /// A list of all wheels in the vessel.
    fn Parts_get_Wheels(this: Class) -> List;

    /// The name of the propellant.
    fn Propellant_get_Name(this: Class) -> String;

    /// The current amount of propellant.
    fn Propellant_get_CurrentAmount(this: Class) -> Double;

    /// The required amount of propellant.
    fn Propellant_get_CurrentRequirement(this: Class) -> Double;

    /// The total amount of the underlying resource currently reachable given
    /// resource flow rules.
    fn Propellant_get_TotalResourceAvailable(this: Class) -> Double;

    /// The total vehicle capacity for the underlying propellant resource,
    /// restricted by resource flow rules.
    fn Propellant_get_TotalResourceCapacity(this: Class) -> Double;

    /// If this propellant should be ignored when calculating required mass flow
    /// given specific impulse.
    fn Propellant_get_IgnoreForIsp(this: Class) -> Bool;

    /// If this propellant should be ignored for thrust curve calculations.
    fn Propellant_get_IgnoreForThrustCurve(this: Class) -> Bool;

    /// If this propellant has a stack gauge or not.
    fn Propellant_get_DrawStackGauge(this: Class) -> Bool;

    /// If this propellant is deprived.
    fn Propellant_get_IsDeprived(this: Class) -> Bool;

    /// The propellant ratio.
    fn Propellant_get_Ratio(this: Class) -> Float;

    /// The part object for this RCS.
    fn RCS_get_Part(this: Class) -> Class;

    /// Whether the RCS thrusters are active.
    /// An RCS thruster is inactive if the RCS action group is disabled
    /// (SpaceCenter.Control.RCS), the RCS thruster itself is not enabled
    /// (SpaceCenter.RCS.Enabled) or it is covered by a fairing
    /// (SpaceCenter.Part.Shielded).
    fn RCS_get_Active(this: Class) -> Bool;

    /// Whether the RCS thrusters are enabled.
    fn RCS_get_Enabled(this: Class) -> Bool;

    /// Whether the RCS thrusters are enabled.
    fn RCS_set_Enabled(this: Class, value: Bool);

    /// Whether the RCS thruster will fire when pitch control input is given.
    fn RCS_get_PitchEnabled(this: Class) -> Bool;

    /// Whether the RCS thruster will fire when pitch control input is given.
    fn RCS_set_PitchEnabled(this: Class, value: Bool);

    /// Whether the RCS thruster will fire when yaw control input is given.
    fn RCS_get_YawEnabled(this: Class) -> Bool;

    /// Whether the RCS thruster will fire when yaw control input is given.
    fn RCS_set_YawEnabled(this: Class, value: Bool);

    /// Whether the RCS thruster will fire when roll control input is given.
    fn RCS_get_RollEnabled(this: Class) -> Bool;

    /// Whether the RCS thruster will fire when roll control input is given.
    fn RCS_set_RollEnabled(this: Class, value: Bool);

    /// Whether the RCS thruster will fire when pitch control input is given.
    fn RCS_get_ForwardEnabled(this: Class) -> Bool;

    /// Whether the RCS thruster will fire when pitch control input is given.
    fn RCS_set_ForwardEnabled(this: Class, value: Bool);

    /// Whether the RCS thruster will fire when yaw control input is given.
    fn RCS_get_UpEnabled(this: Class) -> Bool;

    /// Whether the RCS thruster will fire when yaw control input is given.
    fn RCS_set_UpEnabled(this: Class, value: Bool);

    /// Whether the RCS thruster will fire when roll control input is given.
    fn RCS_get_RightEnabled(this: Class) -> Bool;

    /// Whether the RCS thruster will fire when roll control input is given.
    fn RCS_set_RightEnabled(this: Class, value: Bool);

    /// The available torque, in Newton meters, that can be produced by this
    /// RCS,
    /// in the positive and negative pitch, roll and yaw axes of the vessel.
    /// These axes
    /// correspond to the coordinate axes of the
    /// SpaceCenter.Vessel.ReferenceFrame.
    /// Returns zero if RCS is disable.
    fn RCS_get_AvailableTorque(this: Class) -> Tuple;

    /// The maximum amount of thrust that can be produced by the RCS thrusters
    /// when active,
    /// in Newtons.
    fn RCS_get_MaxThrust(this: Class) -> Float;

    /// The maximum amount of thrust that can be produced by the RCS thrusters
    /// when active
    /// in a vacuum, in Newtons.
    fn RCS_get_MaxVacuumThrust(this: Class) -> Float;

    /// A list of thrusters, one of each nozzel in the RCS part.
    fn RCS_get_Thrusters(this: Class) -> List;

    /// The current specific impulse of the RCS, in seconds. Returns zero
    /// if the RCS is not active.
    fn RCS_get_SpecificImpulse(this: Class) -> Float;

    /// The vacuum specific impulse of the RCS, in seconds.
    fn RCS_get_VacuumSpecificImpulse(this: Class) -> Float;

    /// The specific impulse of the RCS at sea level on Kerbin, in seconds.
    fn RCS_get_KerbinSeaLevelSpecificImpulse(this: Class) -> Float;

    /// The names of resources that the RCS consumes.
    fn RCS_get_Propellants(this: Class) -> List;

    /// The ratios of resources that the RCS consumes. A dictionary mapping
    /// resource names
    /// to the ratios at which they are consumed by the RCS.
    fn RCS_get_PropellantRatios(this: Class) -> Dictionary;

    /// Whether the RCS has fuel available.
    /// 
    /// <remarks>
    /// The RCS thruster must be activated for this property to update
    /// correctly.
    /// </remarks>
    fn RCS_get_HasFuel(this: Class) -> Bool;

    /// The part object for this radiator.
    fn Radiator_get_Part(this: Class) -> Class;

    /// Whether the radiator is deployable.
    fn Radiator_get_Deployable(this: Class) -> Bool;

    /// For a deployable radiator, <c>true</c> if the radiator is extended.
    /// If the radiator is not deployable, this is always <c>true</c>.
    fn Radiator_get_Deployed(this: Class) -> Bool;

    /// For a deployable radiator, <c>true</c> if the radiator is extended.
    /// If the radiator is not deployable, this is always <c>true</c>.
    fn Radiator_set_Deployed(this: Class, value: Bool);

    /// The current state of the radiator.
    /// 
    /// <remarks>
    /// A fixed radiator is always SpaceCenter.RadiatorState.Extended.
    /// </remarks>
    fn Radiator_get_State(this: Class) -> Enumeration;

    /// The part object for this reaction wheel.
    fn ReactionWheel_get_Part(this: Class) -> Class;

    /// Whether the reaction wheel is active.
    fn ReactionWheel_get_Active(this: Class) -> Bool;

    /// Whether the reaction wheel is active.
    fn ReactionWheel_set_Active(this: Class, value: Bool);

    /// Whether the reaction wheel is broken.
    fn ReactionWheel_get_Broken(this: Class) -> Bool;

    /// The available torque, in Newton meters, that can be produced by this
    /// reaction wheel,
    /// in the positive and negative pitch, roll and yaw axes of the vessel.
    /// These axes
    /// correspond to the coordinate axes of the
    /// SpaceCenter.Vessel.ReferenceFrame.
    /// Returns zero if the reaction wheel is inactive or broken.
    fn ReactionWheel_get_AvailableTorque(this: Class) -> Tuple;

    /// The maximum torque, in Newton meters, that can be produced by this
    /// reaction wheel,
    /// when it is active, in the positive and negative pitch, roll and yaw
    /// axes of the vessel.
    /// These axes correspond to the coordinate axes of the
    /// SpaceCenter.Vessel.ReferenceFrame.
    fn ReactionWheel_get_MaxTorque(this: Class) -> Tuple;

    /// True if the specified converter is active.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_Active(this: Class, index: Sint32) -> Bool;

    /// The name of the specified converter.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_Name(this: Class, index: Sint32) -> String;

    /// Start the specified converter.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_Start(this: Class, index: Sint32);

    /// Stop the specified converter.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_Stop(this: Class, index: Sint32);

    /// The state of the specified converter.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_State(this: Class, index: Sint32) -> Enumeration;

    /// Status information for the specified converter.
    /// This is the full status message shown in the in-game UI.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_StatusInfo(this: Class, index: Sint32) -> String;

    /// List of the names of resources consumed by the specified converter.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_Inputs(this: Class, index: Sint32) -> List;

    /// List of the names of resources produced by the specified converter.
    /// 
    /// <param name="index">Index of the converter.</param>
    fn ResourceConverter_Outputs(this: Class, index: Sint32) -> List;

    /// The part object for this converter.
    fn ResourceConverter_get_Part(this: Class) -> Class;

    /// The number of converters in the part.
    fn ResourceConverter_get_Count(this: Class) -> Sint32;

    /// The thermal efficiency of the converter, as a percentage of its maximum.
    fn ResourceConverter_get_ThermalEfficiency(this: Class) -> Float;

    /// The core temperature of the converter, in Kelvin.
    fn ResourceConverter_get_CoreTemperature(this: Class) -> Float;

    /// The core temperature at which the converter will operate with peak
    /// efficiency, in Kelvin.
    fn ResourceConverter_get_OptimumCoreTemperature(this: Class) -> Float;

    /// The part object for this harvester.
    fn ResourceHarvester_get_Part(this: Class) -> Class;

    /// The state of the harvester.
    fn ResourceHarvester_get_State(this: Class) -> Enumeration;

    /// Whether the harvester is deployed.
    fn ResourceHarvester_get_Deployed(this: Class) -> Bool;

    /// Whether the harvester is deployed.
    fn ResourceHarvester_set_Deployed(this: Class, value: Bool);

    /// Whether the harvester is actively drilling.
    fn ResourceHarvester_get_Active(this: Class) -> Bool;

    /// Whether the harvester is actively drilling.
    fn ResourceHarvester_set_Active(this: Class, value: Bool);

    /// The rate at which the drill is extracting ore, in units per second.
    fn ResourceHarvester_get_ExtractionRate(this: Class) -> Float;

    /// The thermal efficiency of the drill, as a percentage of its maximum.
    fn ResourceHarvester_get_ThermalEfficiency(this: Class) -> Float;

    /// The core temperature of the drill, in Kelvin.
    fn ResourceHarvester_get_CoreTemperature(this: Class) -> Float;

    /// The core temperature at which the drill will operate with peak
    /// efficiency, in Kelvin.
    fn ResourceHarvester_get_OptimumCoreTemperature(this: Class) -> Float;

    /// Data amount.
    fn ScienceData_get_DataAmount(this: Class) -> Float;

    /// Science value.
    fn ScienceData_get_ScienceValue(this: Class) -> Float;

    /// Transmit value.
    fn ScienceData_get_TransmitValue(this: Class) -> Float;

    /// Amount of science already earned from this subject, not updated until
    /// after
    /// transmission/recovery.
    fn ScienceSubject_get_Science(this: Class) -> Float;

    /// Total science allowable for this subject.
    fn ScienceSubject_get_ScienceCap(this: Class) -> Float;

    /// Whether the experiment has been completed.
    fn ScienceSubject_get_IsComplete(this: Class) -> Bool;

    /// Multiply science value by this to determine data amount in mits.
    fn ScienceSubject_get_DataScale(this: Class) -> Float;

    /// Diminishing value multiplier for decreasing the science value returned
    /// from repeated
    /// experiments.
    fn ScienceSubject_get_ScientificValue(this: Class) -> Float;

    /// Multiplier for specific Celestial Body/Experiment Situation combination.
    fn ScienceSubject_get_SubjectValue(this: Class) -> Float;

    /// Title of science subject, displayed in science archives
    fn ScienceSubject_get_Title(this: Class) -> String;

    /// The part object for this sensor.
    fn Sensor_get_Part(this: Class) -> Class;

    /// Whether the sensor is active.
    fn Sensor_get_Active(this: Class) -> Bool;

    /// Whether the sensor is active.
    fn Sensor_set_Active(this: Class, value: Bool);

    /// The current value of the sensor.
    fn Sensor_get_Value(this: Class) -> String;

    /// The part object for this solar panel.
    fn SolarPanel_get_Part(this: Class) -> Class;

    /// Whether the solar panel is deployable.
    fn SolarPanel_get_Deployable(this: Class) -> Bool;

    /// Whether the solar panel is extended.
    fn SolarPanel_get_Deployed(this: Class) -> Bool;

    /// Whether the solar panel is extended.
    fn SolarPanel_set_Deployed(this: Class, value: Bool);

    /// The current state of the solar panel.
    fn SolarPanel_get_State(this: Class) -> Enumeration;

    /// The current amount of energy being generated by the solar panel, in
    /// units of charge per second.
    fn SolarPanel_get_EnergyFlow(this: Class) -> Float;

    /// The current amount of sunlight that is incident on the solar panel,
    /// as a percentage. A value between 0 and 1.
    fn SolarPanel_get_SunExposure(this: Class) -> Float;

    /// The position at which the thruster generates thrust, in the given
    /// reference frame.
    /// For gimballed engines, this takes into account the current rotation of
    /// the gimbal.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn Thruster_ThrustPosition(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction of the force generated by the thruster, in the given
    /// reference frame.
    /// This is opposite to the direction in which the thruster expels
    /// propellant.
    /// For gimballed engines, this takes into account the current rotation of
    /// the gimbal.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Thruster_ThrustDirection(this: Class, referenceFrame: Class) -> Tuple;

    /// The position at which the thruster generates thrust, when the engine is
    /// in its
    /// initial position (no gimballing), in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    /// <remarks>
    /// This position can move when the gimbal rotates. This is because the
    /// thrust position and
    /// gimbal position are not necessarily the same.
    /// </remarks>
    fn Thruster_InitialThrustPosition(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction of the force generated by the thruster, when the engine
    /// is in its
    /// initial position (no gimballing), in the given reference frame.
    /// This is opposite to the direction in which the thruster expels
    /// propellant.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Thruster_InitialThrustDirection(this: Class, referenceFrame: Class) -> Tuple;

    /// Position around which the gimbal pivots.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn Thruster_GimbalPosition(this: Class, referenceFrame: Class) -> Tuple;

    /// The <see cref="T:SpaceCenter.Part that contains this thruster.
    fn Thruster_get_Part(this: Class) -> Class;

    /// A reference frame that is fixed relative to the thruster and orientated
    /// with
    /// its thrust direction (SpaceCenter.Thruster.ThrustDirection).
    /// For gimballed engines, this takes into account the current rotation of
    /// the gimbal.
    /// <list type="bullet"><item><description>
    /// The origin is at the position of thrust for this thruster
    /// (SpaceCenter.Thruster.ThrustPosition).</description></item><item><description>
    /// 
    /// The axes rotate with the thrust direction.
    /// This is the direction in which the thruster expels propellant,
    /// including any gimballing.
    /// </description></item><item><description>The y-axis points along the
    /// thrust direction.</description></item><item><description>The x-axis and
    /// z-axis are perpendicular to the thrust direction.
    /// </description></item></list>
    fn Thruster_get_ThrustReferenceFrame(this: Class) -> Class;

    /// Whether the thruster is gimballed.
    fn Thruster_get_Gimballed(this: Class) -> Bool;

    /// The current gimbal angle in the pitch, roll and yaw axes, in degrees.
    fn Thruster_get_GimbalAngle(this: Class) -> Tuple;

    /// The part object for this wheel.
    fn Wheel_get_Part(this: Class) -> Class;

    /// The current state of the wheel.
    fn Wheel_get_State(this: Class) -> Enumeration;

    /// Radius of the wheel, in meters.
    fn Wheel_get_Radius(this: Class) -> Float;

    /// Whether the wheel is touching the ground.
    fn Wheel_get_Grounded(this: Class) -> Bool;

    /// Whether the wheel has brakes.
    fn Wheel_get_HasBrakes(this: Class) -> Bool;

    /// The braking force, as a percentage of maximum, when the brakes are
    /// applied.
    fn Wheel_get_Brakes(this: Class) -> Float;

    /// The braking force, as a percentage of maximum, when the brakes are
    /// applied.
    fn Wheel_set_Brakes(this: Class, value: Float);

    /// Whether automatic friction control is enabled.
    fn Wheel_get_AutoFrictionControl(this: Class) -> Bool;

    /// Whether automatic friction control is enabled.
    fn Wheel_set_AutoFrictionControl(this: Class, value: Bool);

    /// Manual friction control value. Only has an effect if automatic friction
    /// control is disabled.
    /// A value between 0 and 5 inclusive.
    fn Wheel_get_ManualFrictionControl(this: Class) -> Float;

    /// Manual friction control value. Only has an effect if automatic friction
    /// control is disabled.
    /// A value between 0 and 5 inclusive.
    fn Wheel_set_ManualFrictionControl(this: Class, value: Float);

    /// Whether the wheel is deployable.
    fn Wheel_get_Deployable(this: Class) -> Bool;

    /// Whether the wheel is deployed.
    fn Wheel_get_Deployed(this: Class) -> Bool;

    /// Whether the wheel is deployed.
    fn Wheel_set_Deployed(this: Class, value: Bool);

    /// Whether the wheel is powered by a motor.
    fn Wheel_get_Powered(this: Class) -> Bool;

    /// Whether the motor is enabled.
    fn Wheel_get_MotorEnabled(this: Class) -> Bool;

    /// Whether the motor is enabled.
    fn Wheel_set_MotorEnabled(this: Class, value: Bool);

    /// Whether the direction of the motor is inverted.
    fn Wheel_get_MotorInverted(this: Class) -> Bool;

    /// Whether the direction of the motor is inverted.
    fn Wheel_set_MotorInverted(this: Class, value: Bool);

    /// Whether the direction of the motor is inverted.
    fn Wheel_get_MotorState(this: Class) -> Enumeration;

    /// The output of the motor. This is the torque currently being generated,
    /// in Newton meters.
    fn Wheel_get_MotorOutput(this: Class) -> Float;

    /// Whether automatic traction control is enabled.
    /// A wheel only has traction control if it is powered.
    fn Wheel_get_TractionControlEnabled(this: Class) -> Bool;

    /// Whether automatic traction control is enabled.
    /// A wheel only has traction control if it is powered.
    fn Wheel_set_TractionControlEnabled(this: Class, value: Bool);

    /// Setting for the traction control.
    /// Only takes effect if the wheel has automatic traction control enabled.
    /// A value between 0 and 5 inclusive.
    fn Wheel_get_TractionControl(this: Class) -> Float;

    /// Setting for the traction control.
    /// Only takes effect if the wheel has automatic traction control enabled.
    /// A value between 0 and 5 inclusive.
    fn Wheel_set_TractionControl(this: Class, value: Float);

    /// Manual setting for the motor limiter.
    /// Only takes effect if the wheel has automatic traction control disabled.
    /// A value between 0 and 100 inclusive.
    fn Wheel_get_DriveLimiter(this: Class) -> Float;

    /// Manual setting for the motor limiter.
    /// Only takes effect if the wheel has automatic traction control disabled.
    /// A value between 0 and 100 inclusive.
    fn Wheel_set_DriveLimiter(this: Class, value: Float);

    /// Whether the wheel has steering.
    fn Wheel_get_Steerable(this: Class) -> Bool;

    /// Whether the wheel steering is enabled.
    fn Wheel_get_SteeringEnabled(this: Class) -> Bool;

    /// Whether the wheel steering is enabled.
    fn Wheel_set_SteeringEnabled(this: Class, value: Bool);

    /// Whether the wheel steering is inverted.
    fn Wheel_get_SteeringInverted(this: Class) -> Bool;

    /// Whether the wheel steering is inverted.
    fn Wheel_set_SteeringInverted(this: Class, value: Bool);

    /// Whether the wheel has suspension.
    fn Wheel_get_HasSuspension(this: Class) -> Bool;

    /// Suspension spring strength, as set in the editor.
    fn Wheel_get_SuspensionSpringStrength(this: Class) -> Float;

    /// Suspension damper strength, as set in the editor.
    fn Wheel_get_SuspensionDamperStrength(this: Class) -> Float;

    /// Whether the wheel is broken.
    fn Wheel_get_Broken(this: Class) -> Bool;

    /// Whether the wheel is repairable.
    fn Wheel_get_Repairable(this: Class) -> Bool;

    /// Current stress on the wheel.
    fn Wheel_get_Stress(this: Class) -> Float;

    /// Stress tolerance of the wheel.
    fn Wheel_get_StressTolerance(this: Class) -> Float;

    /// Current stress on the wheel as a percentage of its stress tolerance.
    fn Wheel_get_StressPercentage(this: Class) -> Float;

    /// Current deflection of the wheel.
    fn Wheel_get_Deflection(this: Class) -> Float;

    /// Current slip of the wheel.
    fn Wheel_get_Slip(this: Class) -> Float;

    /// Create a relative reference frame. This is a custom reference frame
    /// whose components offset the components of a parent reference frame.
    /// 
    /// <param name="referenceFrame">The parent reference frame on which to
    /// base this reference frame.</param>
    /// <param name="position">The offset of the position of the origin,
    /// as a position vector. Defaults to <math>(0, 0, 0)</math></param>
    /// <param name="rotation">The rotation to apply to the parent frames
    /// rotation,
    /// as a quaternion of the form <math>(x, y, z, w)</math>.
    /// Defaults to <math>(0, 0, 0, 1)</math> (i.e. no rotation)</param>
    /// <param name="velocity">The linear velocity to offset the parent frame
    /// by,
    /// as a vector pointing in the direction of travel, whose magnitude is the
    /// speed in
    /// meters per second. Defaults to <math>(0, 0, 0)</math>.</param>
    /// <param name="angularVelocity">The angular velocity to offset the parent
    /// frame by,
    /// as a vector. This vector points in the direction of the axis of
    /// rotation,
    /// and its magnitude is the speed of the rotation in radians per second.
    /// Defaults to <math>(0, 0, 0)</math>.</param>
    fn ReferenceFrame_static_CreateRelative(referenceFrame: Class, position: Tuple, rotation: Tuple, velocity: Tuple, angularVelocity: Tuple) -> Class;

    /// Create a hybrid reference frame. This is a custom reference frame
    /// whose components inherited from other reference frames.
    /// 
    /// <param name="position">The reference frame providing the position of
    /// the origin.</param>
    /// <param name="rotation">The reference frame providing the rotation of
    /// the frame.</param>
    /// <param name="velocity">The reference frame providing the linear
    /// velocity of the frame.
    /// </param>
    /// <param name="angularVelocity">The reference frame providing the angular
    /// velocity
    /// of the frame.</param>
    /// <remarks>
    /// The <paramref name="position reference frame is required but all other
    /// reference frames are optional. If omitted, they are set to the
    /// <paramref name="position reference frame.
    /// </remarks>
    fn ReferenceFrame_static_CreateHybrid(position: Class, rotation: Class, velocity: Class, angularVelocity: Class) -> Class;

    /// The name of the resource.
    fn Resource_get_Name(this: Class) -> String;

    /// The part containing the resource.
    fn Resource_get_Part(this: Class) -> Class;

    /// The total amount of the resource that can be stored in the part.
    fn Resource_get_Max(this: Class) -> Float;

    /// The amount of the resource that is currently stored in the part.
    fn Resource_get_Amount(this: Class) -> Float;

    /// The density of the resource, in <math>kg/l</math>.
    fn Resource_get_Density(this: Class) -> Float;

    /// The flow mode of the resource.
    fn Resource_get_FlowMode(this: Class) -> Enumeration;

    /// Whether use of this resource is enabled.
    fn Resource_get_Enabled(this: Class) -> Bool;

    /// Whether use of this resource is enabled.
    fn Resource_set_Enabled(this: Class, value: Bool);

    /// Start transferring a resource transfer between a pair of parts. The
    /// transfer will move
    /// at most <paramref name="maxAmount units of the resource, depending on
    /// how much of
    /// the resource is available in the source part and how much storage is
    /// available in the
    /// destination part.
    /// Use SpaceCenter.ResourceTransfer.Complete to check if the transfer is
    /// complete.
    /// Use SpaceCenter.ResourceTransfer.Amount to see how much of the resource
    /// has been transferred.
    /// 
    /// <param name="fromPart">The part to transfer to.</param>
    /// <param name="toPart">The part to transfer from.</param>
    /// <param name="resource">The name of the resource to transfer.</param>
    /// <param name="maxAmount">The maximum amount of resource to
    /// transfer.</param>
    fn ResourceTransfer_static_Start(fromPart: Class, toPart: Class, resource: String, maxAmount: Float) -> Class;

    /// Whether the transfer has completed.
    fn ResourceTransfer_get_Complete(this: Class) -> Bool;

    /// The amount of the resource that has been transferred.
    fn ResourceTransfer_get_Amount(this: Class) -> Float;

    /// All the individual resources with the given name that can be stored.
    fn Resources_WithResource(this: Class, name: String) -> List;

    /// Check whether the named resource can be stored.
    /// 
    /// <param name="name">The name of the resource.</param>
    fn Resources_HasResource(this: Class, name: String) -> Bool;

    /// Returns the amount of a resource that can be stored.
    /// 
    /// <param name="name">The name of the resource.</param>
    fn Resources_Max(this: Class, name: String) -> Float;

    /// Returns the amount of a resource that is currently stored.
    /// 
    /// <param name="name">The name of the resource.</param>
    fn Resources_Amount(this: Class, name: String) -> Float;

    /// Returns the density of a resource, in <math>kg/l</math>.
    /// 
    /// <param name="name">The name of the resource.</param>
    fn Resources_static_Density(name: String) -> Float;

    /// Returns the flow mode of a resource.
    /// 
    /// <param name="name">The name of the resource.</param>
    fn Resources_static_FlowMode(name: String) -> Enumeration;

    /// All the individual resources that can be stored.
    fn Resources_get_All(this: Class) -> List;

    /// A list of resource names that can be stored.
    fn Resources_get_Names(this: Class) -> List;

    /// Whether use of all the resources are enabled.
    /// 
    /// <remarks>
    /// This is <c>true</c> if all of the resources are enabled.
    /// If any of the resources are not enabled, this is <c>false</c>.
    /// </remarks>
    fn Resources_get_Enabled(this: Class) -> Bool;

    /// Whether use of all the resources are enabled.
    /// 
    /// <remarks>
    /// This is <c>true</c> if all of the resources are enabled.
    /// If any of the resources are not enabled, this is <c>false</c>.
    /// </remarks>
    fn Resources_set_Enabled(this: Class, value: Bool);

    /// Recover the vessel.
    fn Vessel_Recover(this: Class);

    /// Returns a <see cref="T:SpaceCenter.Flight object that can be used to
    /// get flight
    /// telemetry for the vessel, in the specified reference frame.
    /// 
    /// <param name="referenceFrame">
    /// Reference frame. Defaults to the vessel's surface reference frame
    /// (SpaceCenter.Vessel.SurfaceReferenceFrame).
    /// </param>
    fn Vessel_Flight(this: Class, referenceFrame: Class) -> Class;

    /// Returns a <see cref="T:SpaceCenter.Resources object, that can used to
    /// get
    /// information about resources stored in a given <paramref name="stage.
    /// 
    /// <param name="stage">Get resources for parts that are decoupled in this
    /// stage.</param>
    /// <param name="cumulative">When <c>false</c>, returns the resources for
    /// parts
    /// decoupled in just the given stage. When <c>true</c> returns the
    /// resources decoupled in
    /// the given stage and all subsequent stages combined.</param>
    fn Vessel_ResourcesInDecoupleStage(this: Class, stage: Sint32, cumulative: Bool) -> Class;

    /// The position of the center of mass of the vessel, in the given
    /// reference frame.
    /// 
    /// # Returns
    /// 
    /// The position as a vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vector is in.</param>
    fn Vessel_Position(this: Class, referenceFrame: Class) -> Tuple;

    /// The axis-aligned bounding box of the vessel in the given reference
    /// frame.
    /// 
    /// # Returns
    /// 
    /// The positions of the minimum and maximum vertices of the box,
    /// as position vectors.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// position vectors are in.</param>
    fn Vessel_BoundingBox(this: Class, referenceFrame: Class) -> Tuple;

    /// The velocity of the center of mass of the vessel, in the given
    /// reference frame.
    /// 
    /// # Returns
    /// 
    /// The velocity as a vector. The vector points in the direction of travel,
    /// and its magnitude is the speed of the body in meters per second.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// velocity vector is in.</param>
    fn Vessel_Velocity(this: Class, referenceFrame: Class) -> Tuple;

    /// The rotation of the vessel, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// rotation is in.</param>
    fn Vessel_Rotation(this: Class, referenceFrame: Class) -> Tuple;

    /// The direction in which the vessel is pointing, in the given reference
    /// frame.
    /// 
    /// # Returns
    /// 
    /// The direction as a unit vector.
    /// 
    /// <param name="referenceFrame">The reference frame that the returned
    /// direction is in.</param>
    fn Vessel_Direction(this: Class, referenceFrame: Class) -> Tuple;

    /// The angular velocity of the vessel, in the given reference frame.
    /// 
    /// # Returns
    /// 
    /// The angular velocity as a vector. The magnitude of the vector is the
    /// rotational
    /// speed of the vessel, in radians per second. The direction of the vector
    /// indicates the
    /// axis of rotation, using the right-hand rule.
    /// 
    /// <param name="referenceFrame">The reference frame the returned
    /// angular velocity is in.</param>
    fn Vessel_AngularVelocity(this: Class, referenceFrame: Class) -> Tuple;

    /// The name of the vessel.
    fn Vessel_get_Name(this: Class) -> String;

    /// The name of the vessel.
    fn Vessel_set_Name(this: Class, value: String);

    /// The type of the vessel.
    fn Vessel_get_Type(this: Class) -> Enumeration;

    /// The type of the vessel.
    fn Vessel_set_Type(this: Class, value: Enumeration);

    /// The situation the vessel is in.
    fn Vessel_get_Situation(this: Class) -> Enumeration;

    /// Whether the vessel is recoverable.
    fn Vessel_get_Recoverable(this: Class) -> Bool;

    /// The mission elapsed time in seconds.
    fn Vessel_get_MET(this: Class) -> Double;

    /// The name of the biome the vessel is currently in.
    fn Vessel_get_Biome(this: Class) -> String;

    /// The current orbit of the vessel.
    fn Vessel_get_Orbit(this: Class) -> Class;

    /// Returns a <see cref="T:SpaceCenter.Control object that can be used to
    /// manipulate
    /// the vessel's control inputs. For example, its pitch/yaw/roll controls,
    /// RCS and thrust.
    fn Vessel_get_Control(this: Class) -> Class;

    /// Returns a <see cref="T:SpaceCenter.Comms object that can be used to
    /// interact
    /// with CommNet for this vessel.
    fn Vessel_get_Comms(this: Class) -> Class;

    /// An <see cref="T:SpaceCenter.AutoPilot object, that can be used to
    /// perform
    /// simple auto-piloting of the vessel.
    fn Vessel_get_AutoPilot(this: Class) -> Class;

    /// The number of crew that can occupy the vessel.
    fn Vessel_get_CrewCapacity(this: Class) -> Sint32;

    /// The number of crew that are occupying the vessel.
    fn Vessel_get_CrewCount(this: Class) -> Sint32;

    /// The crew in the vessel.
    fn Vessel_get_Crew(this: Class) -> List;

    /// A <see cref="T:SpaceCenter.Resources object, that can used to get
    /// information
    /// about resources stored in the vessel.
    fn Vessel_get_Resources(this: Class) -> Class;

    /// A <see cref="T:SpaceCenter.Parts object, that can used to interact with
    /// the parts that make up this vessel.
    fn Vessel_get_Parts(this: Class) -> Class;

    /// The total mass of the vessel, including resources, in kg.
    fn Vessel_get_Mass(this: Class) -> Float;

    /// The total mass of the vessel, excluding resources, in kg.
    fn Vessel_get_DryMass(this: Class) -> Float;

    /// The total thrust currently being produced by the vessel's engines, in
    /// Newtons. This is computed by summing SpaceCenter.Engine.Thrust for
    /// every engine in the vessel.
    fn Vessel_get_Thrust(this: Class) -> Float;

    /// Gets the total available thrust that can be produced by the vessel's
    /// active engines, in Newtons. This is computed by summing
    /// SpaceCenter.Engine.AvailableThrust for every active engine in the
    /// vessel.
    fn Vessel_get_AvailableThrust(this: Class) -> Float;

    /// The total maximum thrust that can be produced by the vessel's active
    /// engines, in Newtons. This is computed by summing
    /// SpaceCenter.Engine.MaxThrust for every active engine.
    fn Vessel_get_MaxThrust(this: Class) -> Float;

    /// The total maximum thrust that can be produced by the vessel's active
    /// engines when the vessel is in a vacuum, in Newtons. This is computed by
    /// summing SpaceCenter.Engine.MaxVacuumThrust for every active engine.
    fn Vessel_get_MaxVacuumThrust(this: Class) -> Float;

    /// The combined specific impulse of all active engines, in seconds. This
    /// is computed using the formula
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>.
    /// 
    fn Vessel_get_SpecificImpulse(this: Class) -> Float;

    /// The combined vacuum specific impulse of all active engines, in seconds.
    /// This is computed using the formula
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>.
    /// 
    fn Vessel_get_VacuumSpecificImpulse(this: Class) -> Float;

    /// The combined specific impulse of all active engines at sea level on
    /// Kerbin, in seconds.
    /// This is computed using the formula
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>.
    /// 
    fn Vessel_get_KerbinSeaLevelSpecificImpulse(this: Class) -> Float;

    /// The moment of inertia of the vessel around its center of mass in
    /// <math>kg.m^2</math>.
    /// The inertia values in the returned 3-tuple are around the
    /// pitch, roll and yaw directions respectively.
    /// This corresponds to the vessels reference frame (<see
    /// cref="T:SpaceCenter.ReferenceFrame).
    fn Vessel_get_MomentOfInertia(this: Class) -> Tuple;

    /// The inertia tensor of the vessel around its center of mass,
    /// in the vessels reference frame (<see
    /// cref="T:SpaceCenter.ReferenceFrame).
    /// Returns the 3x3 matrix as a list of elements, in row-major order.
    fn Vessel_get_InertiaTensor(this: Class) -> List;

    /// The maximum torque that the vessel generates. Includes contributions
    /// from
    /// reaction wheels, RCS, gimballed engines and aerodynamic control
    /// surfaces.
    /// Returns the torques in <math>N.m</math> around each of the coordinate
    /// axes of the
    /// vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    fn Vessel_get_AvailableTorque(this: Class) -> Tuple;

    /// The maximum torque that the currently active and powered reaction
    /// wheels can generate.
    /// Returns the torques in <math>N.m</math> around each of the coordinate
    /// axes of the
    /// vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    fn Vessel_get_AvailableReactionWheelTorque(this: Class) -> Tuple;

    /// The maximum torque that the currently active RCS thrusters can generate.
    /// Returns the torques in <math>N.m</math> around each of the coordinate
    /// axes of the
    /// vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    fn Vessel_get_AvailableRCSTorque(this: Class) -> Tuple;

    /// The maximum torque that the currently active and gimballed engines can
    /// generate.
    /// Returns the torques in <math>N.m</math> around each of the coordinate
    /// axes of the
    /// vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    fn Vessel_get_AvailableEngineTorque(this: Class) -> Tuple;

    /// The maximum torque that the aerodynamic control surfaces can generate.
    /// Returns the torques in <math>N.m</math> around each of the coordinate
    /// axes of the
    /// vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    fn Vessel_get_AvailableControlSurfaceTorque(this: Class) -> Tuple;

    /// The maximum torque that parts (excluding reaction wheels, gimballed
    /// engines,
    /// RCS and control surfaces) can generate.
    /// Returns the torques in <math>N.m</math> around each of the coordinate
    /// axes of the
    /// vessels reference frame (<see cref="T:SpaceCenter.ReferenceFrame).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    fn Vessel_get_AvailableOtherTorque(this: Class) -> Tuple;

    /// The reference frame that is fixed relative to the vessel,
    /// and orientated with the vessel.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// mass of the vessel.</description></item><item><description>The axes
    /// rotate with the vessel.</description></item><item><description>The
    /// x-axis points out to the right of the
    /// vessel.</description></item><item><description>The y-axis points in the
    /// forward direction of the
    /// vessel.</description></item><item><description>The z-axis points out of
    /// the bottom off the vessel.</description></item></list>
    fn Vessel_get_ReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to the vessel,
    /// and orientated with the vessels orbital prograde/normal/radial
    /// directions.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// mass of the vessel.</description></item><item><description>The axes
    /// rotate with the orbital prograde/normal/radial
    /// directions.</description></item><item><description>The x-axis points in
    /// the orbital anti-radial
    /// direction.</description></item><item><description>The y-axis points in
    /// the orbital prograde
    /// direction.</description></item><item><description>The z-axis points in
    /// the orbital normal direction.</description></item></list>
    /// <remarks>
    /// Be careful not to confuse this with 'orbit' mode on the navball.
    /// </remarks>
    fn Vessel_get_OrbitalReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to the vessel,
    /// and orientated with the surface of the body being orbited.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// mass of the vessel.</description></item><item><description>The axes
    /// rotate with the north and up directions on the surface of the
    /// body.</description></item><item><description>The x-axis points in the
    /// <a href="https://en.wikipedia.org/wiki/Zenith">zenith</a>
    /// direction (upwards, normal to the body being orbited, from the center
    /// of the body towards the center of
    /// mass of the vessel).</description></item><item><description>The y-axis
    /// points northwards towards the
    /// <a href="https://en.wikipedia.org/wiki/Horizon">astronomical
    /// horizon</a> (north, and tangential to the
    /// surface of the body -- the direction in which a compass would point
    /// when on the surface).</description></item><item><description>The z-axis
    /// points eastwards towards the
    /// <a href="https://en.wikipedia.org/wiki/Horizon">astronomical
    /// horizon</a> (east, and tangential to the
    /// surface of the body -- east on a compass when on the
    /// surface).</description></item></list>
    /// <remarks>
    /// Be careful not to confuse this with 'surface' mode on the navball.
    /// </remarks>
    fn Vessel_get_SurfaceReferenceFrame(this: Class) -> Class;

    /// The reference frame that is fixed relative to the vessel,
    /// and orientated with the velocity vector of the vessel relative
    /// to the surface of the body being orbited.
    /// <list type="bullet"><item><description>The origin is at the center of
    /// mass of the vessel.</description></item><item><description>The axes
    /// rotate with the vessel's velocity
    /// vector.</description></item><item><description>The y-axis points in the
    /// direction of the vessel's velocity vector,
    /// relative to the surface of the body being
    /// orbited.</description></item><item><description>The z-axis is in the
    /// plane of the
    /// <a href="https://en.wikipedia.org/wiki/Horizon">astronomical
    /// horizon</a>.</description></item><item><description>The x-axis is
    /// orthogonal to the other two axes.</description></item></list>
    fn Vessel_get_SurfaceVelocityReferenceFrame(this: Class) -> Class;

    /// Removes the waypoint.
    fn Waypoint_Remove(this: Class);

    /// The celestial body the waypoint is attached to.
    fn Waypoint_get_Body(this: Class) -> Class;

    /// The celestial body the waypoint is attached to.
    fn Waypoint_set_Body(this: Class, value: Class);

    /// The name of the waypoint as it appears on the map and the contract.
    fn Waypoint_get_Name(this: Class) -> String;

    /// The name of the waypoint as it appears on the map and the contract.
    fn Waypoint_set_Name(this: Class, value: String);

    /// The seed of the icon color. See SpaceCenter.WaypointManager.Colors for
    /// example colors.
    fn Waypoint_get_Color(this: Class) -> Sint32;

    /// The seed of the icon color. See SpaceCenter.WaypointManager.Colors for
    /// example colors.
    fn Waypoint_set_Color(this: Class, value: Sint32);

    /// The icon of the waypoint.
    fn Waypoint_get_Icon(this: Class) -> String;

    /// The icon of the waypoint.
    fn Waypoint_set_Icon(this: Class, value: String);

    /// The latitude of the waypoint.
    fn Waypoint_get_Latitude(this: Class) -> Double;

    /// The latitude of the waypoint.
    fn Waypoint_set_Latitude(this: Class, value: Double);

    /// The longitude of the waypoint.
    fn Waypoint_get_Longitude(this: Class) -> Double;

    /// The longitude of the waypoint.
    fn Waypoint_set_Longitude(this: Class, value: Double);

    /// The altitude of the waypoint above sea level, in meters.
    fn Waypoint_get_MeanAltitude(this: Class) -> Double;

    /// The altitude of the waypoint above sea level, in meters.
    fn Waypoint_set_MeanAltitude(this: Class, value: Double);

    /// The altitude of the waypoint above the surface of the body or sea level,
    /// whichever is closer, in meters.
    fn Waypoint_get_SurfaceAltitude(this: Class) -> Double;

    /// The altitude of the waypoint above the surface of the body or sea level,
    /// whichever is closer, in meters.
    fn Waypoint_set_SurfaceAltitude(this: Class, value: Double);

    /// The altitude of the waypoint above the surface of the body, in meters.
    /// When over water, this is the altitude above the sea floor.
    fn Waypoint_get_BedrockAltitude(this: Class) -> Double;

    /// The altitude of the waypoint above the surface of the body, in meters.
    /// When over water, this is the altitude above the sea floor.
    fn Waypoint_set_BedrockAltitude(this: Class, value: Double);

    /// <c>true</c> if the waypoint is near to the surface of a body.
    fn Waypoint_get_NearSurface(this: Class) -> Bool;

    /// <c>true</c> if the waypoint is attached to the ground.
    fn Waypoint_get_Grounded(this: Class) -> Bool;

    /// The integer index of this waypoint within its cluster of sibling
    /// waypoints.
    /// In other words, when you have a cluster of waypoints called "Somewhere
    /// Alpha",
    /// "Somewhere Beta" and "Somewhere Gamma", the alpha site has index 0, the
    /// beta
    /// site has index 1 and the gamma site has index 2.
    /// When SpaceCenter.Waypoint.Clustered is <c>false</c>, this is zero.
    fn Waypoint_get_Index(this: Class) -> Sint32;

    /// <c>true</c> if this waypoint is part of a set of clustered waypoints
    /// with greek letter
    /// names appended (Alpha, Beta, Gamma, etc).
    /// If <c>true</c>, there is a one-to-one correspondence with the greek
    /// letter name and
    /// the SpaceCenter.Waypoint.Index.
    fn Waypoint_get_Clustered(this: Class) -> Bool;

    /// Whether the waypoint belongs to a contract.
    fn Waypoint_get_HasContract(this: Class) -> Bool;

    /// The associated contract.
    fn Waypoint_get_Contract(this: Class) -> Class;

    /// Creates a waypoint at the given position at ground level, and returns a
    /// <see cref="T:SpaceCenter.Waypoint object that can be used to modify it.
    /// 
    /// <param name="latitude">Latitude of the waypoint.</param>
    /// <param name="longitude">Longitude of the waypoint.</param>
    /// <param name="body">Celestial body the waypoint is attached to.</param>
    /// <param name="name">Name of the waypoint.</param>
    /// # Returns
    fn WaypointManager_AddWaypoint(this: Class, latitude: Double, longitude: Double, body: Class, name: String) -> Class;

    /// Creates a waypoint at the given position and altitude, and returns a
    /// <see cref="T:SpaceCenter.Waypoint object that can be used to modify it.
    /// 
    /// <param name="latitude">Latitude of the waypoint.</param>
    /// <param name="longitude">Longitude of the waypoint.</param>
    /// <param name="altitude">Altitude (above sea level) of the
    /// waypoint.</param>
    /// <param name="body">Celestial body the waypoint is attached to.</param>
    /// <param name="name">Name of the waypoint.</param>
    /// # Returns
    fn WaypointManager_AddWaypointAtAltitude(this: Class, latitude: Double, longitude: Double, altitude: Double, body: Class, name: String) -> Class;

    /// A list of all existing waypoints.
    fn WaypointManager_get_Waypoints(this: Class) -> List;

    /// Returns all available icons (from "GameData/Squad/Contracts/Icons/").
    fn WaypointManager_get_Icons(this: Class) -> List;

    /// An example map of known color - seed pairs.
    /// Any other integers may be used as seed.
    fn WaypointManager_get_Colors(this: Class) -> Dictionary;

}

/// Provides functionality for drawing and interacting with in-game user
/// interface elements.
/// 
/// <remarks>
/// For drawing 3D objects in the flight scene, see the Drawing service.
/// </remarks>
mod UI {
    /// A text label. See UI.Panel.AddButton.
    struct Button;

    /// A canvas for user interface elements. See UI.StockCanvas and
    /// UI.AddCanvas.
    struct Canvas;

    /// An input field. See UI.Panel.AddInputField.
    struct InputField;

    /// A container for user interface elements. See UI.Canvas.AddPanel.
    struct Panel;

    /// A Unity engine Rect Transform for a UI object.
    /// See the <a
    /// href="https://docs.unity3d.com/Manual/class-RectTransform.html">Unity
    /// manual</a> for more details.
    struct RectTransform;

    /// A text label. See UI.Panel.AddText.
    struct Text;

    /// Font style.
    enum FontStyle {
        /// Normal.
        Normal = 0,
        /// Bold.
        Bold = 1,
        /// Italic.
        Italic = 2,
        /// Bold and italic.
        BoldAndItalic = 3,
    }

    /// Message position.
    enum MessagePosition {
        /// Bottom center.
        BottomCenter = 0,
        /// Top center.
        TopCenter = 1,
        /// Top left.
        TopLeft = 2,
        /// Top right.
        TopRight = 3,
    }

    /// Text alignment.
    enum TextAlignment {
        /// Left aligned.
        Left = 0,
        /// Right aligned.
        Right = 1,
        /// Center aligned.
        Center = 2,
    }

    /// Text alignment.
    enum TextAnchor {
        /// Lower center.
        LowerCenter = 0,
        /// Lower left.
        LowerLeft = 1,
        /// Lower right.
        LowerRight = 2,
        /// Middle center.
        MiddleCenter = 3,
        /// Middle left.
        MiddleLeft = 4,
        /// Middle right.
        MiddleRight = 5,
        /// Upper center.
        UpperCenter = 6,
        /// Upper left.
        UpperLeft = 7,
        /// Upper right.
        UpperRight = 8,
    }

    /// Add a new canvas.
    /// 
    /// <remarks>
    /// If you want to add UI elements to KSPs stock UI canvas, use
    /// UI.StockCanvas.
    /// </remarks>
    fn AddCanvas() -> Class;

    /// Display a message on the screen.
    /// 
    /// <remarks>
    /// The message appears just like a stock message, for example quicksave or
    /// quickload messages.
    /// </remarks>
    /// <param name="content">Message content.</param>
    /// <param name="duration">Duration before the message disappears, in
    /// seconds.</param>
    /// <param name="position">Position to display the message.</param>
    /// <param name="size">Size of the message, differs per position.</param>
    /// <param name="color">The color of the message.</param>
    fn Message(content: String, duration: Float, position: Enumeration, color: Tuple, size: Float);

    /// Remove all user interface elements.
    /// 
    /// <param name="clientOnly">If true, only remove objects created by the
    /// calling client.</param>
    fn Clear(clientOnly: Bool);

    /// The stock UI canvas.
    fn get_StockCanvas() -> Class;

    /// Remove the UI object.
    fn Button_Remove(this: Class);

    /// The rect transform for the text.
    fn Button_get_RectTransform(this: Class) -> Class;

    /// The text for the button.
    fn Button_get_Text(this: Class) -> Class;

    /// Whether the button has been clicked.
    /// 
    /// <remarks>
    /// This property is set to true when the user clicks the button.
    /// A client script should reset the property to false in order to detect
    /// subsequent button presses.
    /// </remarks>
    fn Button_get_Clicked(this: Class) -> Bool;

    /// Whether the button has been clicked.
    /// 
    /// <remarks>
    /// This property is set to true when the user clicks the button.
    /// A client script should reset the property to false in order to detect
    /// subsequent button presses.
    /// </remarks>
    fn Button_set_Clicked(this: Class, value: Bool);

    /// Whether the UI object is visible.
    fn Button_get_Visible(this: Class) -> Bool;

    /// Whether the UI object is visible.
    fn Button_set_Visible(this: Class, value: Bool);

    /// Create a new container for user interface elements.
    /// 
    /// <param name="visible">Whether the panel is visible.</param>
    fn Canvas_AddPanel(this: Class, visible: Bool) -> Class;

    /// Add text to the canvas.
    /// 
    /// <param name="content">The text.</param>
    /// <param name="visible">Whether the text is visible.</param>
    fn Canvas_AddText(this: Class, content: String, visible: Bool) -> Class;

    /// Add an input field to the canvas.
    /// 
    /// <param name="visible">Whether the input field is visible.</param>
    fn Canvas_AddInputField(this: Class, visible: Bool) -> Class;

    /// Add a button to the canvas.
    /// 
    /// <param name="content">The label for the button.</param>
    /// <param name="visible">Whether the button is visible.</param>
    fn Canvas_AddButton(this: Class, content: String, visible: Bool) -> Class;

    /// Remove the UI object.
    fn Canvas_Remove(this: Class);

    /// The rect transform for the canvas.
    fn Canvas_get_RectTransform(this: Class) -> Class;

    /// Whether the UI object is visible.
    fn Canvas_get_Visible(this: Class) -> Bool;

    /// Whether the UI object is visible.
    fn Canvas_set_Visible(this: Class, value: Bool);

    /// Remove the UI object.
    fn InputField_Remove(this: Class);

    /// The rect transform for the input field.
    fn InputField_get_RectTransform(this: Class) -> Class;

    /// The value of the input field.
    fn InputField_get_Value(this: Class) -> String;

    /// The value of the input field.
    fn InputField_set_Value(this: Class, value: String);

    /// The text component of the input field.
    /// 
    /// <remarks>
    /// Use UI.InputField.Value to get and set the value in the field.
    /// This object can be used to alter the style of the input field's text.
    /// </remarks>
    fn InputField_get_Text(this: Class) -> Class;

    /// Whether the input field has been changed.
    /// 
    /// <remarks>
    /// This property is set to true when the user modifies the value of the
    /// input field.
    /// A client script should reset the property to false in order to detect
    /// subsequent changes.
    /// </remarks>
    fn InputField_get_Changed(this: Class) -> Bool;

    /// Whether the input field has been changed.
    /// 
    /// <remarks>
    /// This property is set to true when the user modifies the value of the
    /// input field.
    /// A client script should reset the property to false in order to detect
    /// subsequent changes.
    /// </remarks>
    fn InputField_set_Changed(this: Class, value: Bool);

    /// Whether the UI object is visible.
    fn InputField_get_Visible(this: Class) -> Bool;

    /// Whether the UI object is visible.
    fn InputField_set_Visible(this: Class, value: Bool);

    /// Create a panel within this panel.
    /// 
    /// <param name="visible">Whether the new panel is visible.</param>
    fn Panel_AddPanel(this: Class, visible: Bool) -> Class;

    /// Add text to the panel.
    /// 
    /// <param name="content">The text.</param>
    /// <param name="visible">Whether the text is visible.</param>
    fn Panel_AddText(this: Class, content: String, visible: Bool) -> Class;

    /// Add an input field to the panel.
    /// 
    /// <param name="visible">Whether the input field is visible.</param>
    fn Panel_AddInputField(this: Class, visible: Bool) -> Class;

    /// Add a button to the panel.
    /// 
    /// <param name="content">The label for the button.</param>
    /// <param name="visible">Whether the button is visible.</param>
    fn Panel_AddButton(this: Class, content: String, visible: Bool) -> Class;

    /// Remove the UI object.
    fn Panel_Remove(this: Class);

    /// The rect transform for the panel.
    fn Panel_get_RectTransform(this: Class) -> Class;

    /// Whether the UI object is visible.
    fn Panel_get_Visible(this: Class) -> Bool;

    /// Whether the UI object is visible.
    fn Panel_set_Visible(this: Class, value: Bool);

    /// Position of the rectangles pivot point relative to the anchors.
    fn RectTransform_get_Position(this: Class) -> Tuple;

    /// Position of the rectangles pivot point relative to the anchors.
    fn RectTransform_set_Position(this: Class, value: Tuple);

    /// Position of the rectangles pivot point relative to the anchors.
    fn RectTransform_get_LocalPosition(this: Class) -> Tuple;

    /// Position of the rectangles pivot point relative to the anchors.
    fn RectTransform_set_LocalPosition(this: Class, value: Tuple);

    /// Width and height of the rectangle.
    fn RectTransform_get_Size(this: Class) -> Tuple;

    /// Width and height of the rectangle.
    fn RectTransform_set_Size(this: Class, value: Tuple);

    /// Position of the rectangles upper right corner relative to the anchors.
    fn RectTransform_get_UpperRight(this: Class) -> Tuple;

    /// Position of the rectangles upper right corner relative to the anchors.
    fn RectTransform_set_UpperRight(this: Class, value: Tuple);

    /// Position of the rectangles lower left corner relative to the anchors.
    fn RectTransform_get_LowerLeft(this: Class) -> Tuple;

    /// Position of the rectangles lower left corner relative to the anchors.
    fn RectTransform_set_LowerLeft(this: Class, value: Tuple);

    /// Set the minimum and maximum anchor points as a fraction of the size of
    /// the parent rectangle.
    fn RectTransform_set_Anchor(this: Class, value: Tuple);

    /// The anchor point for the lower left corner of the rectangle defined as
    /// a fraction of the size of the parent rectangle.
    fn RectTransform_get_AnchorMax(this: Class) -> Tuple;

    /// The anchor point for the lower left corner of the rectangle defined as
    /// a fraction of the size of the parent rectangle.
    fn RectTransform_set_AnchorMax(this: Class, value: Tuple);

    /// The anchor point for the upper right corner of the rectangle defined as
    /// a fraction of the size of the parent rectangle.
    fn RectTransform_get_AnchorMin(this: Class) -> Tuple;

    /// The anchor point for the upper right corner of the rectangle defined as
    /// a fraction of the size of the parent rectangle.
    fn RectTransform_set_AnchorMin(this: Class, value: Tuple);

    /// Location of the pivot point around which the rectangle rotates, defined
    /// as a fraction of the size of the rectangle itself.
    fn RectTransform_get_Pivot(this: Class) -> Tuple;

    /// Location of the pivot point around which the rectangle rotates, defined
    /// as a fraction of the size of the rectangle itself.
    fn RectTransform_set_Pivot(this: Class, value: Tuple);

    /// Rotation, as a quaternion, of the object around its pivot point.
    fn RectTransform_get_Rotation(this: Class) -> Tuple;

    /// Rotation, as a quaternion, of the object around its pivot point.
    fn RectTransform_set_Rotation(this: Class, value: Tuple);

    /// Scale factor applied to the object in the x, y and z dimensions.
    fn RectTransform_get_Scale(this: Class) -> Tuple;

    /// Scale factor applied to the object in the x, y and z dimensions.
    fn RectTransform_set_Scale(this: Class, value: Tuple);

    /// Remove the UI object.
    fn Text_Remove(this: Class);

    /// The rect transform for the text.
    fn Text_get_RectTransform(this: Class) -> Class;

    /// A list of all available fonts.
    fn Text_get_AvailableFonts(this: Class) -> List;

    /// The text string
    fn Text_get_Content(this: Class) -> String;

    /// The text string
    fn Text_set_Content(this: Class, value: String);

    /// Name of the font
    fn Text_get_Font(this: Class) -> String;

    /// Name of the font
    fn Text_set_Font(this: Class, value: String);

    /// Font size.
    fn Text_get_Size(this: Class) -> Sint32;

    /// Font size.
    fn Text_set_Size(this: Class, value: Sint32);

    /// Font style.
    fn Text_get_Style(this: Class) -> Enumeration;

    /// Font style.
    fn Text_set_Style(this: Class, value: Enumeration);

    /// Alignment.
    fn Text_get_Alignment(this: Class) -> Enumeration;

    /// Alignment.
    fn Text_set_Alignment(this: Class, value: Enumeration);

    /// Line spacing.
    fn Text_get_LineSpacing(this: Class) -> Float;

    /// Line spacing.
    fn Text_set_LineSpacing(this: Class, value: Float);

    /// Set the color
    fn Text_get_Color(this: Class) -> Tuple;

    /// Set the color
    fn Text_set_Color(this: Class, value: Tuple);

    /// Whether the UI object is visible.
    fn Text_get_Visible(this: Class) -> Bool;

    /// Whether the UI object is visible.
    fn Text_set_Visible(this: Class, value: Bool);

}

